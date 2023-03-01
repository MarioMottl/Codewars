use num::Integer;

fn proper_fractions(n: u64) -> u64 {
    (1..n).into_iter().filter(|i| n.gcd(i) == 1).count().try_into().unwrap()
}


fn main() {
    proper_fractions(15);
    //proper_fractions(25);
}

#[cfg(test)]
mod tests {
    use super::proper_fractions;


    #[test]
    fn sample_tests() {
        assert_eq!(proper_fractions(1), 0, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(2), 1, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(5), 4, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(15), 8, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(proper_fractions(25), 20, "\nYour answer (left) is not the expected answer (right).");
    }
}