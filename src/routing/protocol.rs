use serde::{Deserialize, Serialize};

/// ScanRequest represents a content discovery request in the SCAN protocol.
/// It is sent alongside traditional IP routing requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub content_name: String,
    pub source_node: String,
    pub hop_count: u32,
    pub max_hops: u32,
    pub is_priority: bool,
}

impl ScanRequest {
    pub fn new(content_name: String, source_node: String, is_priority: bool) -> Self {
        Self {
            content_name,
            source_node,
            hop_count: 0,
            max_hops: 5, // Default depth from paper
            is_priority,
        }
    }

    pub fn with_depth(mut self, depth: u32) -> Self {
        self.max_hops = depth;
        self
    }

    pub fn increment_hop(&mut self) {
        self.hop_count += 1;
    }
}

/// ScanResponse represents the result of a SCAN discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScanResponse {
    Found {
        node_id: String,
        latency_estimate: u32,
    },
    NotFound,
    Error(String),
}
