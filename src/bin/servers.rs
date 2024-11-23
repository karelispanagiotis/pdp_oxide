use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        k: usize,
    }

    let mut hist = vec![0; m];
    for _ in 0..m {
        for _ in 0..n {
            input! { page: usize, visits: i32 };
            hist[page] += visits;
        }
    }

    let mut indices: Vec<usize> = (0..m).collect();
    indices.sort_unstable_by(|&i, &j| hist[j].cmp(&hist[i]));

    for i in 0..k {
        println!("{} {}", indices[i], hist[indices[i]]);
    }
}
