// Run with `cargo +nightly bench` once a bench harness is wired in. The single-file
// micro-benchmark below is here so future contributors can see what to measure.

#![cfg(test)]
use repla::hash::state_root;
use std::time::Instant;

#[test]
fn state_root_is_under_microseconds_per_kbyte() {
    let payload = vec![0xABu8; 1024];
    let refs: Vec<&[u8]> = vec![payload.as_slice(); 16];
    let start = Instant::now();
    for _ in 0..200 {
        let _ = state_root(&refs);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 200, "state_root too slow: {:?}", elapsed);
}
