fn solution(num: i32) -> i32 {
    let mut numbers = (1..num).filter(|i| i % 3 == 0 || i % 5  == 0);
    numbers.fold(0, |acc, x| acc + x as i32)
}
fn main() {
    println!("{:}", solution(10));
}

mod tests {
    use super::solution;
    
    #[test]
    fn sample_tests() {
      // assertion(expected, input);
      assertion(  23,   10);
      assertion(  33,   11);
      assertion( 225,   33);
      assertion(   8,    6);
      assertion(3420,  123);
      assertion( 543,   50);
      assertion(   0,    0);
      assertion(   0, -203);
      assertion(25719750, 10500);
    }
    
    fn assertion(expected : i32, input : i32) {
        let actual = solution(input);
        
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n", expected, actual, input
        );
    }
}