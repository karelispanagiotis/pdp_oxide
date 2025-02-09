use proconio::input;

fn main() {
    input! { n: usize, prices: [i32; n] };

    let mut suffix_max = prices.clone();
    for i in (0..n - 1).rev() {
        suffix_max[i] = std::cmp::max(suffix_max[i], suffix_max[i + 1]);
    }

    let mut ans = 1.0;
    for i in 0..n {
        ans = f64::max(suffix_max[i] as f64 / prices[i] as f64, ans);
    }

    println!("{:.3}", ans);
}
