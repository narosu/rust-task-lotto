use core::num;
use std::{env, mem::take};

use rand::{Rng, prelude::{IteratorRandom, SliceRandom}, thread_rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        //Lotto {take,from,numbers:self.get}
        
    }

    fn get_numbers(self) -> Vec<usize> {
        let mut numbers:Vec<usize> = Vec::new();
        let mut rng = thread_rng();
        let nrpool:Vec<usize> = (1..self.from).collect();
        
        for _x in 1..self.take {
            numbers.push(*nrpool.choose(&mut rng).unwrap());
        }
        println!("{:?}",&self.numbers);
        return numbers
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    // Tip: Use the format macro
    todo!("Implement")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}, {}", &args[1], &args[2]);
    let take:usize = args[1].parse().expect("Could not parse value for takes");
    let from:usize = args[2].parse().expect("Could not parse value for numbers");
    let lotto = Lotto::new(take,from);
    lotto.numbers = lotto.get_numbers();
    println!("{:?}",&lotto.numbers);
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.get_numbers();
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}
