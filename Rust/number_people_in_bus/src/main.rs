fn number(bus_stops:&[(i32,i32)]) -> i32 {

    for stop in bus_stops {
        println!("{:?}{:?}", stop.0, stop.1)
    }

    return 1;
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}