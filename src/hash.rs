use sha2::{Digest, Sha256};

/// Canonical state-root hash. The wire format is one bytestream: for each payload
/// `p`, write `(p.len() as u32).to_le_bytes()` then `p` itself.
pub fn state_root(payloads: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for p in payloads {
        hasher.update((p.len() as u32).to_le_bytes());
        hasher.update(p);
    }
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}
