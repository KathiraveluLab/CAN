use std::sync::Arc;
use std::path::Path;
use dashmap::DashMap;
use redb::{Database, TableDefinition};
use crate::core::content::ContentMetadata;

const LCT_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("lct");

/// Local Content Table (LCT)
/// Manages the persistent local cache index for the C-router using redb.
pub struct LocalContentTable {
    db: Arc<Database>,
}

impl LocalContentTable {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let db = Database::builder()
            .create(path)
            .expect("Failed to create LCT database");
        
        // Ensure table exists
        let write_txn = db.begin_write().expect("Failed to start write transaction");
        {
            write_txn.open_table(LCT_TABLE).expect("Failed to open table");
        }
        write_txn.commit().expect("Failed to commit initial table creation");

        Self { db: Arc::new(db) }
    }

    pub fn insert(&self, metadata: ContentMetadata) {
        let write_txn = self.db.begin_write().expect("Failed to start write transaction");
        {
            let mut table = write_txn.open_table(LCT_TABLE).expect("Failed to open table");
            let encoded = rmp_serde::to_vec(&metadata).expect("Failed to encode metadata");
            table.insert(metadata.name.as_str(), encoded.as_slice()).expect("Failed to insert into LCT");
        }
        write_txn.commit().expect("Failed to commit LCT write");
    }

    pub fn get(&self, name: &str) -> Option<Arc<ContentMetadata>> {
        let read_txn = self.db.begin_read().expect("Failed to start read transaction");
        let table = read_txn.open_table(LCT_TABLE).expect("Failed to open table");
        let value = table.get(name).expect("Failed to get from LCT")?;
        let metadata: ContentMetadata = rmp_serde::from_slice(value.value()).expect("Failed to decode metadata");
        Some(Arc::new(metadata))
    }

    pub fn contains(&self, name: &str) -> bool {
        let read_txn = self.db.begin_read().expect("Failed to start read transaction");
        let table = read_txn.open_table(LCT_TABLE).expect("Failed to open table");
        table.get(name).expect("Failed to check LCT").is_some()
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
