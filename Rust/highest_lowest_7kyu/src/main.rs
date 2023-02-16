fn high_and_low(numbers: &str) -> String {

    let iter = numbers.split_whitespace().map(|num| num.parse::<i32>().unwrap());

    let min = iter.clone().min().unwrap();
    let max = iter.clone().max().unwrap();
    println!("max: {:}, min: {:}", max, min);
    return format!("{:} {:}", max, min);

}
fn main() {
    high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
}

#[test]
fn example_test_1() {
  assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
  assert_eq!("3 1", high_and_low("1 2 3"));
}
