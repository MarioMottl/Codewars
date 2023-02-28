fn int32_to_ip(int: u32) -> String{
    let int_as_bytes: [u8; 4] = int.to_be_bytes();
    let output = int_as_bytes.map(|i| i.to_string()).join(".");
    output

}

fn main() {
    int32_to_ip(2154959208);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
        assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
        assert_eq!(int32_to_ip(0), "0.0.0.0");
    }
}