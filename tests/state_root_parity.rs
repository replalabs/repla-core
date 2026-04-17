use repla::hash::state_root;

#[test]
fn empty_input_matches_sha256_of_nothing() {
    let r = state_root(&[]);
    let hex: String = r.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(hex, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}
