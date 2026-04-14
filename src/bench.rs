use std::time::{Instant, Duration};
use fastbloom::BloomFilter;
use std::collections::HashSet;
use crate::core::tables::LocalContentTable;
use crate::core::content::{ContentMetadata, ContentType};

pub struct BenchResult {
    pub name: String,
    pub iterations: u32,
    pub total_duration: Duration,
    pub avg_latency: Duration,
}

pub async fn run_benchmarks() {
    println!("\n[PERFORMANCE EVALUATION] Starting Micro-benchmarks");
    
    // 1. Bloom Filter vs HashSet Performance
    bench_bloom_vs_hashset();
    
    // 2. Persistent LCT Latency
    bench_lct_latency().await;
}

fn bench_bloom_vs_hashset() {
    let size = 10_000;
    let mut hashset = HashSet::new();
    let mut bloom = BloomFilter::with_num_bits(64 * 1024).expected_items(size);
    
    for i in 0..size {
        let key = format!("content-id-{}", i);
        hashset.insert(key.clone());
        bloom.insert(&key);
    }
    
    // Benchmark Queries
    let iterations = 100_000;
    
    // HashSet
    let start_hs = Instant::now();
    for i in 0..iterations {
        let key = format!("content-id-{}", i % size);
        let _ = hashset.contains(&key);
    }
    let dur_hs = start_hs.elapsed();
    
    // Bloom
    let start_bl = Instant::now();
    for i in 0..iterations {
        let key = format!("content-id-{}", i % size);
        let _ = bloom.contains(&key);
    }
    let dur_bl = start_bl.elapsed();
    
    println!("--- Probabilistic Indexing Performance ---");
    println!("HashSet lookup ({} items, {} queries): {:?}", size, iterations, dur_hs);
    println!("Bloom Filter lookup ({} items, {} queries): {:?}", size, iterations, dur_bl);
    println!("Relative Speedup: {:.2}x", dur_hs.as_secs_f64() / dur_bl.as_secs_f64());
}

async fn bench_lct_latency() {
    let node_id = "bench-node".to_string();
    let db_path = "bench-node_cache.redb";
    let _ = std::fs::remove_file(db_path);
    
    let lct = LocalContentTable::new(node_id.clone());
    let size = 1000;
    
    // Ingestion Timing
    let start_ingest = Instant::now();
    for i in 0..size {
        let meta = ContentMetadata::new(
            format!("path/to/item-{}", i),
            ContentType::Generic,
            5,
            1024
        );
        lct.insert(meta);
    }
    let dur_ingest = start_ingest.elapsed();
    
    // Lookup Timing
    let iterations = 2000;
    let start_lookup = Instant::now();
    for i in 0..iterations {
        let _ = lct.get(&format!("path/to/item-{}", i % size));
    }
    let dur_lookup = start_lookup.elapsed();
    
    println!("\n--- Persistent Storage Performance (redb) ---");
    println!("Ingestion Latency (Avg for {} items): {:?}", size, dur_ingest / size as u32);
    println!("Lookup Latency (Avg for {} queries): {:?}", iterations, dur_lookup / iterations as u32);
    
    let _ = std::fs::remove_file(db_path);
}
