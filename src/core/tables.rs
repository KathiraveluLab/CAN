use std::sync::Arc;
use dashmap::DashMap;
use crate::core::content::ContentMetadata;

/// Local Content Table (LCT)
/// Manages the local cache index for the C-router.
pub struct LocalContentTable {
    items: DashMap<String, Arc<ContentMetadata>>,
}

impl LocalContentTable {
    pub fn new() -> Self {
        Self {
            items: DashMap::new(),
        }
    }

    pub fn insert(&self, metadata: ContentMetadata) {
        self.items.insert(metadata.name.clone(), Arc::new(metadata));
    }

    pub fn get(&self, name: &str) -> Option<Arc<ContentMetadata>> {
        self.items.get(name).map(|r| Arc::clone(r.value()))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.items.contains_key(name)
    }
}

impl Default for LocalContentTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Content Routing Table (CRT)
/// Manages information about content cached in neighboring nodes.
pub struct ContentRoutingTable {
    /// Maps content Name (or ID) to a list of Node IDs that have the content.
    routes: DashMap<String, Vec<String>>,
}

impl ContentRoutingTable {
    pub fn new() -> Self {
        Self {
            routes: DashMap::new(),
        }
    }

    pub fn add_route(&self, content_name: String, node_id: String) {
        let mut entry = self.routes.entry(content_name).or_default();
        if !entry.contains(&node_id) {
            entry.push(node_id);
        }
    }

    pub fn get_nodes(&self, content_name: &str) -> Option<Vec<String>> {
        self.routes.get(content_name).map(|r| r.value().clone())
    }
}

impl Default for ContentRoutingTable {
    fn default() -> Self {
        Self::new()
    }
}
