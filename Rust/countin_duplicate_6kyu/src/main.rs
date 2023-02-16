fn count_duplicates(text: &str) -> u32 {
    let mut counter: u32 = 0;
    for char in  text.chars(){
        let c = text.matches(char).count();
        if c > 0 {
            counter += 1;
        }
    }
    return counter;
}

fn main() {
    println!("{:?}", count_duplicates("abcde")) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }
}