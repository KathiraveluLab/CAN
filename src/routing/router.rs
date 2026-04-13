use crate::core::bloom::BloomFilterWrapper;
use crate::core::tables::{ContentRoutingTable, LocalContentTable};
use crate::routing::protocol::{ScanRequest, ScanResponse};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, instrument};
use async_recursion::async_recursion;

/// CRouter (Content-Router)
/// The core engine of the CAN framework. It performs content-aware routing by
/// intercepting ScanRequests and performing multi-level lookups.
pub struct CRouter {
    pub node_id: String,
    pub lct: LocalContentTable,
    pub crt: ContentRoutingTable,
    pub bloom_filter: Arc<RwLock<BloomFilterWrapper>>,
    /// Neighbors involved in the network tree for recursive searching.
    /// In a production system, this would be network interfaces/sockets.
    pub neighbors: RwLock<Vec<Arc<CRouter>>>,
}

impl CRouter {
    pub fn new(node_id: String) -> Self {
        let db_path = format!("{}_cache.redb", node_id);
        Self {
            node_id,
            lct: LocalContentTable::new(db_path),
            crt: ContentRoutingTable::new(),
            bloom_filter: Arc::new(RwLock::new(BloomFilterWrapper::new(1000))),
            neighbors: RwLock::new(Vec::new()),
        }
    }

    pub async fn add_neighbor(&self, neighbor: Arc<CRouter>) {
        self.neighbors.write().await.push(neighbor);
    }

    /// Handles an incoming ScanRequest using the SCAN protocol logic:
    /// 1. Check Local Content Table (LCT)
    /// 2. Check Content Routing Table (CRT) for neighboring caches
    /// 3. Consult Bloom Filter for recursive "Network Tree Search"
    /// 4. Fallback to traditional IP routing
    #[async_recursion]
    #[instrument(skip(self), fields(node = %self.node_id))]
    pub async fn handle_scan_request(&self, mut request: ScanRequest) -> ScanResponse {
        info!(
            content = %request.content_name,
            priority = %request.is_priority,
            hop = %request.hop_count,
            "Intercepted scan request"
        );

        // Step 1: Check LCT (Local Content Table) - Persistent Disk Cache
        if self.lct.contains(&request.content_name) {
            info!("LCT HIT: serving content locally");
            return ScanResponse::Found {
                node_id: self.node_id.clone(),
                latency_estimate: 1, 
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

        // Step 3: Check Bloom Filter (Probabilistic verification for Tree Search)
        let bf = self.bloom_filter.read().await;
        if bf.contains(&request.content_name) {
            info!("Bloom Filter Match: Triggering recursive network tree search");
            
            request.increment_hop();
            if request.hop_count < request.max_hops {
                let neighbors = self.neighbors.read().await;
                for neighbor in neighbors.iter() {
                    // Avoid sending back to the source node
                    if neighbor.node_id == request.source_node {
                        continue;
                    }
                    
                    // Propagate search deeper into the tree
                    let res = neighbor.handle_scan_request(request.clone()).await;
                    if let ScanResponse::Found { .. } = res {
                        info!(found_at = ?res, "Recursive Search SUCCESS");
                        return res;
                    }
                }
            }
        }

        // Step 4: Logic for Priority (Healthcare Use Case)
        if request.is_priority {
            info!("Priority request flagged: Broader search range or elevated TTL would apply here");
        }

        request.increment_hop();
        if request.hop_count >= request.max_hops {
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
    use std::fs;

    fn cleanup(node_id: &str) {
        let _ = fs::remove_file(format!("{}_cache.redb", node_id));
    }

    #[tokio::test]
    async fn test_persistence_and_hit() {
        let node_id = "persistence-test";
        cleanup(node_id);
        
        {
            let router = CRouter::new(node_id.to_string());
            let meta = ContentMetadata::new(
                "test-content".to_string(),
                ContentType::Generic,
                5,
                100,
            );
            router.cache_content(meta).await;
        }

        // Re-initialize to test persistence
        let router = CRouter::new(node_id.to_string());
        let req = ScanRequest::new("test-content".to_string(), "requester".to_string(), false);
        let res = router.handle_scan_request(req).await;

        match res {
            ScanResponse::Found { node_id: res_node, .. } => assert_eq!(res_node, node_id),
            _ => panic!("Expected LCT HIT after restart"),
        }
        cleanup(node_id);
    }
}
