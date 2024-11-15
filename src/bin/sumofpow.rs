use proconio::input;
use std::vec;

fn pows_to_n(n: i32, pow: i32) -> Vec<i32> {
    let mut pows = vec![1];
    let mut last = pows.last().unwrap();
    while *last < n {
        pows.push(*last * pow);
        last = pows.last().unwrap();
    }
    pows
}

fn main() {
    input! { n: i32 };
    let pows2 = pows_to_n(n, 2);
    let pows3 = pows_to_n(n, 3);
    let pows5 = pows_to_n(n, 5);

    let mut all_possible = Vec::new();
    for pow5 in &pows5 {
        for pow3 in &pows3 {
            for pow2 in &pows2 {
                dbg!(pow2, pow3, pow5, pow2 + pow3 + pow5);
                all_possible.push(pow5 + pow3 + pow2);
            }
        }
    }
    all_possible.sort_unstable();
    all_possible.dedup();
    println!("{}", all_possible.partition_point(|&x| x <= n));
}
