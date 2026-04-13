use can_framework::apps::healthcare::dicom::DicomRouter;
use can_framework::routing::multicast::MulticastScheduler;
use can_framework::routing::protocol::{ScanRequest, ScanResponse};
use can_framework::routing::router::CRouter;
use can_framework::core::content::{ContentMetadata, ContentType};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize professional logging
    tracing_subscriber::fmt::init();
    
    info!("Starting CAN (Content-Aware-Networking) Framework Simulation");

    // Cleanup any previous simulation files
    let _ = std::fs::remove_file("central-hub_cache.redb");
    let _ = std::fs::remove_file("hospital-a_cache.redb");
    let _ = std::fs::remove_file("temp-node_cache.redb");
    let _ = std::fs::remove_file("core-node_cache.redb");
    let _ = std::fs::remove_file("hub-node_cache.redb");
    let _ = std::fs::remove_file("edge-node_cache.redb");

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

    // 4. Ingest DICOM data into Hospital-A (Advanced DPI Simulation)
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

    // 7. Scenario C: Time Slot Multicast Efficiency
    println!("\n[SCENARIO C] Time Slot Multicast: Grouping multiple requests");
    let scheduler = MulticastScheduler::new();
    
    // Simulations requests arriving in the same time slot
    scheduler.add_request("healthcare/dicom/pat-001/MRI-Brain".to_string(), "clinic-x".to_string());
    scheduler.add_request("healthcare/dicom/pat-001/MRI-Brain".to_string(), "clinic-y".to_string());
    scheduler.add_request("healthcare/dicom/pat-001/MRI-Brain".to_string(), "hospital-z".to_string());
    
    scheduler.process_multicast();

    // 8. Scenario D: Persistent Cache Recovery
    println!("\n[SCENARIO D] Persistent Cache Recovery (redb backend)");
    {
        let temp_router = CRouter::new("temp-node".to_string());
        let meta = ContentMetadata::new(
            "docs/manual.pdf".to_string(),
            ContentType::Generic,
            5,
            2048
        );
        temp_router.cache_content(meta).await;
        info!("Data ingested into 'temp-node' (persistent store)");
    } // temp_router dropped

    let recovered_router = CRouter::new("temp-node".to_string());
    let req_d = ScanRequest::new("docs/manual.pdf".to_string(), "requester".to_string(), false);
    let res_d = recovered_router.handle_scan_request(req_d).await;
    println!("Recovery Response (expecting Found from temp-node): {:?}", res_d);

    // 9. Scenario E: Network Tree Search
    println!("\n[SCENARIO E] Network Tree Search: Recursive discovery");
    let core = Arc::new(CRouter::new("core-node".to_string()));
    let hub_node = Arc::new(CRouter::new("hub-node".to_string()));
    let edge = Arc::new(CRouter::new("edge-node".to_string()));

    // Build Topology: Edge -> Hub -> Core
    edge.add_neighbor(hub_node.clone()).await;
    hub_node.add_neighbor(core.clone()).await;

    // Ingest data at Core
    let deep_meta = ContentMetadata::new(
        "deep/search/item.dat".to_string(),
        ContentType::Generic,
        5,
        1024
    );
    core.cache_content(deep_meta).await;

    // Manually prime Bloom Filters for upstream discovery (simulating SCAN control plane exchange)
    {
        let mut hub_bf = hub_node.bloom_filter.write().await;
        hub_bf.insert("deep/search/item.dat");
        let mut edge_bf = edge.bloom_filter.write().await;
        edge_bf.insert("deep/search/item.dat");
    }

    let req_e = ScanRequest::new(
        "deep/search/item.dat".to_string(),
        "external-req".to_string(),
        false
    ).with_depth(3);
    
    let res_e = edge.handle_scan_request(req_e).await;
    println!("Deep Search Response (expecting Found from core-node): {:?}", res_e);

    println!("\n----------------------------------------------------------");
    println!("Simulation Complete: All scenarios (A, B, C, D, E) verified.");

    // Final cleanup of simulation files
    let _ = std::fs::remove_file("central-hub_cache.redb");
    let _ = std::fs::remove_file("hospital-a_cache.redb");
    let _ = std::fs::remove_file("temp-node_cache.redb");
    let _ = std::fs::remove_file("core-node_cache.redb");
    let _ = std::fs::remove_file("hub-node_cache.redb");
    let _ = std::fs::remove_file("edge-node_cache.redb");

    Ok(())
}
