use crate::routing::router::CRouter;
use crate::core::content::{ContentMetadata, ContentType};
use tracing::info;

/// DicomRouter
/// A specialized router implementation for the healthcare use case described
/// in Section IV of the paper. It handles DICOM (Digital Imaging and Communications 
/// in Medicine) data with high priority.
pub struct DicomRouter {
    inner: CRouter,
}

impl DicomRouter {
    pub fn new(node_id: String) -> Self {
        Self {
            inner: CRouter::new(node_id),
        }
    }

    /// Simulates the ingestion of a radiographic image (X-ray, MRI, etc.)
    /// Marks it as highest importance (10) to trigger priority content-aware routing.
    pub async fn ingest_radiology_data(&self, patient_id: &str, modality: &str, size_bytes: u64) {
        let content_name = format!("healthcare/dicom/{}/{}", patient_id, modality);
        
        let metadata = ContentMetadata::new(
            content_name.clone(),
            ContentType::HealthcareRadiology,
            10, // Max importance for radiology data
            size_bytes,
        );

        info!(
            patient = %patient_id, 
            modality = %modality, 
            "DICOM Ingestion: Prioritizing radiographic image for content-aware delivery"
        );
        
        self.inner.cache_content(metadata).await;
    }

    /// Accessor for the inner CRouter logic
    pub fn router(&self) -> &CRouter {
        &self.inner
    }
}
