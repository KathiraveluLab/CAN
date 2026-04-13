use dashmap::DashMap;
use std::sync::Arc;
use tracing::info;

/// Represents a group of nodes waiting for the same content in a time slot.
pub struct MulticastGroup {
    pub content_name: String,
    pub members: Vec<String>,
}

/// Schedules and groups content requests into time-slot based multicast broadcasts.
pub struct MulticastScheduler {
    groups: DashMap<String, Vec<String>>,
}

impl MulticastScheduler {
    pub fn new() -> Self {
        Self {
            groups: DashMap::new(),
        }
    }

    /// Adds a node to the multicast group for specific content.
    pub fn add_request(&self, content_name: String, source_node: String) {
        let mut group = self.groups.entry(content_name).or_insert(Vec::new());
        group.push(source_node);
    }

    /// Processes all buffered groups and returns the multicast events.
    /// In a real system, this would be triggered by a timer.
    pub fn flush(&self) -> Vec<MulticastGroup> {
        let mut results = Vec::new();
        let mut keys_to_remove = Vec::new();

        for entry in self.groups.iter() {
            results.push(MulticastGroup {
                content_name: entry.key().clone(),
                members: entry.value().clone(),
            });
            keys_to_remove.push(entry.key().clone());
        }

        for key in keys_to_remove {
            self.groups.remove(&key);
        }

        results
    }

    pub fn process_multicast(&self) {
        let batches = self.flush();
        for batch in batches {
            if batch.members.len() > 1 {
                info!(
                    "MULTICAST BROADCAST: Deliving content '{}' to group of {} users: {:?}",
                    batch.content_name,
                    batch.members.len(),
                    batch.members
                );
            } else if !batch.members.is_empty() {
                info!(
                    "UNICAST DELIVERY: Single user {:?} served for content '{}'",
                    batch.members, batch.content_name
                );
            }
        }
    }
}

impl Default for MulticastScheduler {
    fn default() -> Self {
        Self::new()
    }
}
