// new term:
// associated type
// iterator adaptor

use std::path::Iter;

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let total: i32 = v1.iter().sum();
    assert_eq!(total, 6);

    let mut v1_iter_2 = v1.into_iter();

    assert_eq!(v1_iter_2.next(), Some(1));
    assert_eq!(v1_iter_2.next(), Some(2));
    assert_eq!(v1_iter_2.next(), Some(3));
    assert_eq!(v1_iter_2.next(), None);

    // error : value borrowed here after move
    // let total: i32 = v1.iter().sum();
    // assert_eq!(total, 6);
}

#[test]
fn test_iterator_adapters() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let counter = Counter::new();
    for count in counter.into_iter() {
        println!("{}", count);
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().into_iter())
        .map(|(a, b)| a * b)
        .sum();

    assert_eq!(55, sum);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);

}

#[test]
fn test1() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v1);
    println!("{:?}", v2);
}