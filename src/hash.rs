use sha2::{Digest, Sha256};

/// Canonical state-root hash. Both the Rust sequencer and the TypeScript SDK must
/// produce byte-identical output for the same input order, so the wire is one bytestream:
/// for each payload `p`, write `(p.len() as u32).to_le_bytes()` then `p` itself.
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

pub fn settle_signing_message(
    l3_id: &[u8; 32],
    from_slot: u64,
    to_slot: u64,
    root: &[u8; 32],
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(l3_id);
    hasher.update(from_slot.to_le_bytes());
    hasher.update(to_slot.to_le_bytes());
    hasher.update(root);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn root_is_deterministic(seq in proptest::collection::vec(any::<Vec<u8>>(), 0..16)) {
            let refs: Vec<&[u8]> = seq.iter().map(|s| s.as_slice()).collect();
            let a = state_root(&refs);
            let b = state_root(&refs);
            prop_assert_eq!(a, b);
        }

        #[test]
        fn root_depends_on_order(a in any::<Vec<u8>>(), b in any::<Vec<u8>>()) {
            prop_assume!(a != b);
            let ra = state_root(&[a.as_slice(), b.as_slice()]);
            let rb = state_root(&[b.as_slice(), a.as_slice()]);
            prop_assert_ne!(ra, rb);
        }
    }

    #[test]
    fn empty_root_is_sha256_of_nothing() {
        let r = state_root(&[]);
        let expected_hex = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
        let got_hex: String = r.iter().map(|b| format!("{:02x}", b)).collect();
        assert_eq!(got_hex, expected_hex);
    }
}
