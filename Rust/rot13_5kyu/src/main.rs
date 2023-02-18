fn rot13(message: &str) -> String {
    let mut coded_string = String::from("");
    for c in message.chars() {
        let charcode = c as u32;
        if c.is_lowercase() {
            let a_code = 'a' as u32;
            let rotcode = ((charcode - a_code + 13) % 26) + a_code;
            coded_string.push(char::from_u32(rotcode).unwrap());
        } else if c.is_uppercase() {
            let a_code = 'A' as u32;
            let rotcode = ((charcode - a_code + 13) % 26) + a_code;
            coded_string.push(char::from_u32(rotcode).unwrap());
        } else {
            coded_string.push(c);
        }
    }
    coded_string

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::rot13;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: &str, expected: &str) {
        assert_eq!(rot13(s), expected, "{ERR_MSG} with message = \"{s}\"")
    }

    #[test]
    fn sample_tests() {
        dotest("test", "grfg");
        dotest("Test", "Grfg");
    }
}