use crate::routing::router::CRouter;
use crate::core::content::{ContentMetadata, ContentType, RadiologyMetadata};
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
    /// Peforms simulated "Advanced DPI" by extracting granular metadata fields.
    pub async fn ingest_radiology_data(&self, patient_id: &str, modality: &str, size_bytes: u64) {
        let radiology = RadiologyMetadata {
            study_uid: format!("1.2.840.113619.2.134.{}", uuid::Uuid::new_v4()),
            series_description: modality.to_string(),
            slice_thickness: 1.0,
            instance_number: 1,
            patient_id: patient_id.to_string(),
        };

        let content_name = format!("healthcare/dicom/{}/{}", patient_id, modality);
        
        let metadata = ContentMetadata::new(
            content_name.clone(),
            ContentType::HealthcareRadiology(radiology),
            10, // Max importance for radiology data
            size_bytes,
        );

        info!(
            patient = %patient_id, 
            modality = %modality, 
            "DICOM Ingestion: Prioritizing radiographic image with granular DPI inspection"
        );
        
        self.inner.cache_content(metadata).await;
    }

    /// Accessor for the inner CRouter logic
    pub fn router(&self) -> &CRouter {
        &self.inner
    }
}
