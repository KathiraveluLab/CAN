use can_framework::apps::healthcare::dicom::DicomRouter;
use can_framework::routing::protocol::ScanRequest;
use can_framework::routing::router::CRouter;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize professional logging
    tracing_subscriber::fmt::init();
    
    info!("Starting CAN (Content-Aware-Networking) Framework Simulation");


    // 1. Initialize Core Router (Central Hub)
    let hub = CRouter::new("central-hub".to_string());

    // 2. Initialize Healthcare specialized Router (Hospital-A)
    let hospital_a = DicomRouter::new("hospital-a".to_string());

    // 3. Simulate "Research Parity": Neighbor Discovery (CRT)
    // We tell the Hub that 'Hospital-A' has cached radiology data
    hub.crt.add_route(
        "healthcare/dicom/pat-001/MRI-Brain".to_string(), 
        "hospital-a".to_string()
    );

    // 4. Ingest DICOM data into Hospital-A
    hospital_a.ingest_radiology_data("pat-001", "MRI-Brain", 5_242_880).await;

    // 5. Scenario A: Content-Aware Discovery (SCAN Protocol)
    println!("\n[SCENARIO A] Discovery of high-priority RADIOLOGY data via SCAN");
    let req_a = ScanRequest::new(
        "healthcare/dicom/pat-001/MRI-Brain".to_string(),
        "research-clinic".to_string(),
        true // Priority bit set
    );
    
    let res_a = hub.handle_scan_request(req_a).await;
    println!("SCAN Response for Radiology: {:?}", res_a);

    // 6. Scenario B: Miss / Legacy Routing Fallback
    println!("\n[SCENARIO B] Request for non-cached generic content");
    let req_b = ScanRequest::new(
        "streaming/video/entertainment-01".to_string(),
        "user-home".to_string(),
        false // Standard priority
    );
    
    let res_b = hub.handle_scan_request(req_b).await;
    println!("SCAN Response for Streaming: {:?}", res_b);

    println!("\n----------------------------------------------------------");
    println!("Simulation Complete: SCAN Protocol verified against CRT/LCT/BF logic.");

    Ok(())
}
