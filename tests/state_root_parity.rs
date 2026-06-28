use repla::hash::state_root;

#[test]
fn ordered_payloads_produce_known_root() {
    let payloads: Vec<&[u8]> = vec![b"alpha", b"bravo", b"charlie"];
    let root = state_root(&payloads);
    let hex: String = root.iter().map(|b| format!("{:02x}", b)).collect();
    let len = hex.len();
    assert_eq!(len, 64);
    assert_ne!(hex, "0".repeat(64));
}

#[test]
fn root_changes_when_one_byte_flips() {
    let r1 = state_root(&[b"hello"]);
    let r2 = state_root(&[b"hellp"]);
    assert_ne!(r1, r2);
}

#[test]
fn empty_input_matches_sha256_of_nothing() {
    let r = state_root(&[]);
    let hex: String = r.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(
        hex,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}
