fn main() {
    println!("Hop Count, Standard IP (ms), CAN (Cache Hit) (ms), CAN (Cache Miss) (ms)");
    for hops in 1..=10 {
        let ip_latency = hops as f64 * 10.0;
        let can_hit = 0.66; // Sub-millisecond local discovery
        let can_miss = (hops as f64 * 10.0) + 1.5; // Includes search overhead
        println!("{}, {:.2}, {:.2}, {:.2}", hops, ip_latency, can_hit, can_miss);
    }
}
