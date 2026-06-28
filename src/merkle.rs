use sha2::{Digest, Sha256};

/// Domain-separation tag prepended before hashing a leaf. Together with
/// [`NODE_DOMAIN`] this defends the tree against second-preimage attacks: an
/// internal node digest can never be reinterpreted as a leaf digest because the
/// two are hashed under different prefixes. This is the same construction used
/// by RFC 6962 (Certificate Transparency).
const LEAF_DOMAIN: u8 = 0x00;

/// Domain-separation tag prepended before hashing an internal node.
const NODE_DOMAIN: u8 = 0x01;

/// Hash a raw action payload into a 32-byte Merkle leaf.
///
/// The encoding mirrors the length-prefixed convention in [`crate::hash`]: the
/// leaf domain byte, then `(payload.len() as u32).to_le_bytes()`, then the
/// payload itself. Because every leaf carries the leaf domain tag, no leaf can
/// collide with an internal node, and the explicit length prefix keeps the
/// encoding unambiguous across the Rust sequencer and the TypeScript SDK.
pub fn leaf_hash(payload: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([LEAF_DOMAIN]);
    hasher.update((payload.len() as u32).to_le_bytes());
    hasher.update(payload);
    finalize(hasher)
}

/// Hash two child digests into their parent under the internal-node domain.
fn node_hash(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update([NODE_DOMAIN]);
    hasher.update(left);
    hasher.update(right);
    finalize(hasher)
}

/// The root returned for an empty leaf set: SHA-256 of the empty byte string.
///
/// This matches [`crate::hash::state_root`] of an empty payload list, so an L3
/// that has settled no actions reports the same canonical root through either
/// hashing path.
fn empty_root() -> [u8; 32] {
    finalize(Sha256::new())
}

/// Compute the binary Merkle root of `leaves`.
///
/// Each input leaf is first hashed under [`leaf_hash`] to form the base level,
/// then adjacent digests are folded with [`node_hash`] until a single root
/// remains. On a level with an odd node count the final node is paired with a
/// copy of itself (Bitcoin-style duplication). An empty input yields
/// [`empty_root`].
///
/// The root commits to both the contents and the order of the leaves, so two
/// sequencers that batch the same actions in different orders produce different
/// roots and are distinguishable on settlement.
pub fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return empty_root();
    }

    let mut level: Vec<[u8; 32]> = leaves.iter().map(|leaf| leaf_hash(leaf)).collect();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for pair in level.chunks(2) {
            let left = &pair[0];
            let right = if pair.len() == 2 { &pair[1] } else { &pair[0] };
            next.push(node_hash(left, right));
        }
        level = next;
    }
    level[0]
}

/// Build the sibling authentication path for the leaf at `index`.
///
/// The returned vector lists one sibling digest per tree level, bottom to top.
/// Feeding it to [`verify_merkle_proof`] together with the original leaf and the
/// root reconstructs the root and proves membership. An out-of-range index
/// yields an empty path (which only verifies against a single-leaf tree, and
/// not against the requested index otherwise).
pub fn merkle_proof(leaves: &[[u8; 32]], index: usize) -> Vec<[u8; 32]> {
    let mut proof = Vec::new();
    if index >= leaves.len() {
        return proof;
    }

    let mut level: Vec<[u8; 32]> = leaves.iter().map(|leaf| leaf_hash(leaf)).collect();
    let mut idx = index;
    while level.len() > 1 {
        let sibling_idx = if idx.is_multiple_of(2) {
            idx + 1
        } else {
            idx - 1
        };
        // On an odd-length level the last node is paired with itself, so its
        // sibling is its own digest.
        let sibling = if sibling_idx < level.len() {
            level[sibling_idx]
        } else {
            level[idx]
        };
        proof.push(sibling);

        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        for pair in level.chunks(2) {
            let left = &pair[0];
            let right = if pair.len() == 2 { &pair[1] } else { &pair[0] };
            next.push(node_hash(left, right));
        }
        level = next;
        idx /= 2;
    }
    proof
}

/// Verify that `leaf` sits at `index` under `root` given its sibling `proof`.
///
/// `leaf` is the raw 32-byte leaf value (the same bytes passed to
/// [`merkle_root`]); the leaf domain hash is applied internally. The parity of
/// `index` at each level decides whether the running digest is the left or right
/// child, so a proof generated for one position cannot be replayed at another.
pub fn verify_merkle_proof(
    root: [u8; 32],
    leaf: [u8; 32],
    index: usize,
    proof: &[[u8; 32]],
) -> bool {
    let mut computed = leaf_hash(&leaf);
    let mut idx = index;
    for sibling in proof {
        computed = if idx.is_multiple_of(2) {
            node_hash(&computed, sibling)
        } else {
            node_hash(sibling, &computed)
        };
        idx /= 2;
    }
    computed == root
}

/// Read a finished SHA-256 hasher into a fixed 32-byte array.
fn finalize(hasher: Sha256) -> [u8; 32] {
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(bytes: &[u8; 32]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    // Independently reproduced with Python's hashlib (sha256) over the same
    // domain-separated, length-prefixed encoding. See the module docs.
    const LEAF_A: [u8; 32] = [0x01; 32];
    const LEAF_B: [u8; 32] = [0x02; 32];
    const LEAF_C: [u8; 32] = [0x03; 32];

    #[test]
    fn root_is_deterministic() {
        let leaves = [LEAF_A, LEAF_B, LEAF_C];
        assert_eq!(merkle_root(&leaves), merkle_root(&leaves));
    }

    #[test]
    fn empty_root_is_sha256_of_nothing() {
        assert_eq!(
            hex(&merkle_root(&[])),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn single_leaf_root_is_leaf_domain_hash() {
        // A one-leaf tree has no internal nodes, so its root is exactly the
        // leaf domain hash of that leaf.
        let root = merkle_root(&[LEAF_A]);
        assert_eq!(root, leaf_hash(&LEAF_A));
        assert_eq!(
            hex(&root),
            "fbc1621a6bb7e4ec785ea605d82067383f7071dab31bd6d4c0ac2aaedcf82f17"
        );
    }

    #[test]
    fn two_leaf_root_matches_known_vector() {
        let root = merkle_root(&[LEAF_A, LEAF_B]);
        assert_eq!(
            hex(&root),
            "9a542c4e68f1c9dc07823b9eaee9cda7ca8b8af55d6483cb7fead1b597bb722e"
        );
    }

    #[test]
    fn three_leaf_root_matches_known_vector() {
        // Odd level: the third leaf is paired with a copy of itself.
        let root = merkle_root(&[LEAF_A, LEAF_B, LEAF_C]);
        assert_eq!(
            hex(&root),
            "1adb0eb61ca20a5dc73bb8c2b921b0e591e059a75da4dbba87ab1187d9bfc8cd"
        );
    }

    #[test]
    fn leaf_hash_matches_known_vector() {
        assert_eq!(
            hex(&leaf_hash(b"action-7")),
            "184ed2f826e4bc671eea2da1de06b1490d6e2f3d57b5014a614d270c8ed5029b"
        );
    }

    #[test]
    fn proof_verifies_for_every_index() {
        // Cover odd and even leaf counts so the self-pairing path is exercised.
        for count in 1..=9usize {
            let leaves: Vec<[u8; 32]> = (0..count).map(|i| [i as u8; 32]).collect();
            let root = merkle_root(&leaves);
            for index in 0..count {
                let proof = merkle_proof(&leaves, index);
                assert!(
                    verify_merkle_proof(root, leaves[index], index, &proof),
                    "membership proof failed for {index} of {count}"
                );
            }
        }
    }

    #[test]
    fn tampered_leaf_fails_verification() {
        let leaves = [LEAF_A, LEAF_B, LEAF_C];
        let root = merkle_root(&leaves);
        let proof = merkle_proof(&leaves, 1);
        let mut forged = LEAF_B;
        forged[0] ^= 0xff;
        assert!(!verify_merkle_proof(root, forged, 1, &proof));
    }

    #[test]
    fn tampered_proof_fails_verification() {
        let leaves = [LEAF_A, LEAF_B, LEAF_C, LEAF_A];
        let root = merkle_root(&leaves);
        let mut proof = merkle_proof(&leaves, 2);
        assert!(!proof.is_empty());
        proof[0][0] ^= 0x01;
        assert!(!verify_merkle_proof(root, leaves[2], 2, &proof));
    }

    #[test]
    fn wrong_index_fails_verification() {
        let leaves = [LEAF_A, LEAF_B, LEAF_C, LEAF_A];
        let root = merkle_root(&leaves);
        let proof = merkle_proof(&leaves, 1);
        // Correct leaf and proof, wrong claimed position.
        assert!(verify_merkle_proof(root, leaves[1], 1, &proof));
        assert!(!verify_merkle_proof(root, leaves[1], 0, &proof));
    }

    #[test]
    fn root_is_order_sensitive() {
        let forward = merkle_root(&[LEAF_A, LEAF_B, LEAF_C]);
        let shuffled = merkle_root(&[LEAF_C, LEAF_B, LEAF_A]);
        assert_ne!(forward, shuffled);
    }
}
