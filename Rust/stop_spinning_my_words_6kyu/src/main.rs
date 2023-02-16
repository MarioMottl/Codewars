fn spin_words(words: &str) -> String {
    let mut new_string: String = "".to_string();
    println!("INPUT: {:?}", words);
    for word in words.split_whitespace() {
        if word.len() >= 5 {
            new_string.push_str(&word.chars().rev().collect::<String>())
        }
        else {
            new_string.push_str(word);
        }
        if words.contains(" "){
            new_string.push_str(" ");
        }
    }
    if new_string.ends_with(" ") {
        new_string = new_string[0..new_string.len()-1].to_string()
    }
    return new_string;

    /*
        words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
        }).collect::<Vec<String>>().join(" ")
    */

}

fn main() {
    println!("{:?}",spin_words("Welcome"));
    println!("{:?}",spin_words("You are almost to the last test"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(spin_words("You are almost to the last test"), "You are tsomla to the last test");
        assert_eq!(spin_words("Just kidding there is still one more"), "Just gniddik ereht is llits one more");
        assert_eq!(spin_words("Seriously this is the last one"), "ylsuoireS this is the last one");
    }
}