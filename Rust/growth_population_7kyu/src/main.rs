fn nb_year(p0: i32, percent: f64, aug: i32, p: i32)-> i32 {
    // p0 = Population start
    // percent = percentage of growth
    // aug = increase of population
    // expected population

    //formular = p0 + (p0 * percent) + aug
    let mut population: i32 = p0;
    let mut years: i32 = 1;

    let percentage = percent / 100.0;

    while population < p {
        population = population + (population as f64 * percentage) as i32 + aug;
        println!("{:}", population);
        if population <= p {
            years = years + 1;
        }
    }

    return years;
}

fn main() {
    println!("{:?}", nb_year(1500, 5.0, 100, 5000));
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(p0: i32, percent: f64, aug: i32, p: i32, exp: i32) -> () {
        println!("p0: {:?};", p0);
        println!("percent: {:?};", percent);
        println!("aug: {:?};", aug);
        println!("p: {:?};", p);
        let ans =nb_year(p0, percent, aug, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(1500, 5.0, 100, 5000, 15);
        dotest(1500000, 2.5, 10000, 2000000, 10);
        
    }
}