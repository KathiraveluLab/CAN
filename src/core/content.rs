use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RadiologyMetadata {
    pub study_uid: String,
    pub series_description: String,
    pub slice_thickness: f32,
    pub instance_number: u32,
    pub patient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Generic,
    HealthcareRadiology(RadiologyMetadata),
    HealthcareRecords,
    VideoStreaming,
    WebText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub id: String,
    pub name: String,
    pub content_type: ContentType,
    pub importance: u8, // 1 (low) to 10 (high)
    pub size_bytes: u64,
}

impl ContentMetadata {
    pub fn new(name: String, content_type: ContentType, importance: u8, size_bytes: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            content_type,
            importance,
            size_bytes,
        }
    }
}
