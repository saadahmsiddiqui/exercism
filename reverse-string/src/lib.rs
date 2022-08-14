pub fn reverse(input: &str) -> String {
    let as_bts = input.as_bytes();
    let mut bts_vec = Vec::from(as_bts);
    bts_vec.reverse();

    String::from_utf8(bts_vec).unwrap()
}
