// stolen from stack overflow
fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars()
        .map(|c| format!("{:b}", u32::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect()
}

fn main() {
    let mut data = convert_to_binary_from_hex(include_str!("input0"));
    let version = &data[0..=2];
    let type_id = &data[3..=5];

}
