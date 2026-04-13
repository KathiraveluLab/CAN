use crate::core::bloom::BloomFilterWrapper;
use crate::core::tables::{ContentRoutingTable, LocalContentTable};
use crate::routing::protocol::{ScanRequest, ScanResponse};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, instrument};

/// CRouter (Content-Router)
/// The core engine of the CAN framework. It performs content-aware routing by
/// intercepting ScanRequests and performing multi-level lookups.
pub struct CRouter {
    pub node_id: String,
    pub lct: LocalContentTable,
    pub crt: ContentRoutingTable,
    pub bloom_filter: Arc<RwLock<BloomFilterWrapper>>,
}

impl CRouter {
    pub fn new(node_id: String) -> Self {
        Self {
            node_id,
            lct: LocalContentTable::new(),
            crt: ContentRoutingTable::new(),
            bloom_filter: Arc::new(RwLock::new(BloomFilterWrapper::new(1000))),
        }
    }

    /// Handles an incoming ScanRequest using the SCAN protocol logic:
    /// 1. Check Local Content Table (LCT)
    /// 2. Check Content Routing Table (CRT) for neighboring caches
    /// 3. Consult Bloom Filter for probabilistic match
    /// 4. Fallback to traditional IP routing
    #[instrument(skip(self), fields(node = %self.node_id))]
    pub async fn handle_scan_request(&self, mut request: ScanRequest) -> ScanResponse {
        info!(
            content = %request.content_name,
            priority = %request.is_priority,
            "Intercepted scan request"
        );

        // Step 1: Check LCT (Local Content Table)
        if self.lct.contains(&request.content_name) {
            info!("LCT HIT: serving content locally");
            return ScanResponse::Found {
                node_id: self.node_id.clone(),
                latency_estimate: 1, // Nominal local latency
            };
        }

        // Step 2: Check CRT (Content Routing Table) for neighboring copies
        if let Some(nodes) = self.crt.get_nodes(&request.content_name) {
            if let Some(best_neighbor) = nodes.first() {
                info!(neighbor = %best_neighbor, "CRT HIT: redirecting to neighboring cache");
                return ScanResponse::Found {
                    node_id: best_neighbor.clone(),
                    latency_estimate: 10 * (request.hop_count + 1),
                };
            }
        }

        // Step 3: Check Bloom Filter (Probabilistic verification)
        let bf = self.bloom_filter.read().await;
        if bf.contains(&request.content_name) {
            info!("Bloom Filter Match: Content importance justifies deep search");
            // In a full implementation, this would trigger a recursive search
        }

        // Step 4: Logic for Priority (Healthcare Use Case)
        if request.is_priority {
            info!("Priority request: bypassing standard debounce logic");
            // High-priority (Healthcare) traffic could have higher TTL or broader search
        }

        request.increment_hop();
        if request.hop_count > 5 {
            warn!("Max hop limit reached; terminating content search");
            return ScanResponse::NotFound;
        }

        // Default: Fallback to traditional IP routing (simulated as NotFound here)
        info!("SCAN MISS: Falling back to legacy IP routing");
        ScanResponse::NotFound
    }

    /// Caches content locally and updates the LCT and Bloom Filter.
    pub async fn cache_content(&self, metadata: crate::core::content::ContentMetadata) {
        let name = metadata.name.clone();
        self.lct.insert(metadata);
        
        // Update Bloom Filter for probabilistic indexing
        let mut bf = self.bloom_filter.write().await;
        bf.insert(&name);
        
        info!(content = %name, "Content successfully cached and indexed in Bloom Filter");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::content::{ContentMetadata, ContentType};

    #[tokio::test]
    async fn test_lct_hit() {
        let router = CRouter::new("test-node".to_string());
        let meta = ContentMetadata::new(
            "test-content".to_string(),
            ContentType::Generic,
            5,
            100,
        );
        router.cache_content(meta).await;

        let req = ScanRequest::new("test-content".to_string(), "requester".to_string(), false);
        let res = router.handle_scan_request(req).await;

        match res {
            ScanResponse::Found { node_id, .. } => assert_eq!(node_id, "test-node"),
            _ => panic!("Expected LCT HIT"),
        }
    }

    #[tokio::test]
    async fn test_crt_hit() {
        let router = CRouter::new("test-node".to_string());
        router.crt.add_route("neighbor-content".to_string(), "neighbor-node".to_string());

        let req = ScanRequest::new("neighbor-content".to_string(), "requester".to_string(), false);
        let res = router.handle_scan_request(req).await;

        match res {
            ScanResponse::Found { node_id, .. } => assert_eq!(node_id, "neighbor-node"),
            _ => panic!("Expected CRT HIT"),
        }
    }
}
