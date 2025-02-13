use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, rank: [usize; n] };

    let mut final_ranking = Vec::with_capacity(n);
    for (i, rnk) in rank.iter().enumerate() {
        final_ranking.insert(*rnk - 1, i);
    }

    let mut ans = vec![0; n];
    for (i, entry) in final_ranking.iter().enumerate() {
        ans[*entry] = i + 1;
    }

    for x in ans {
        println!("{}", x);
    }
}
