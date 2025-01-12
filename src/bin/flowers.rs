use proconio::{input, read_value};

fn main() {
    input! { n: usize };
    let mut t = vec![0; n];
    let mut d = vec![0; n];
    for i in 0..n {
        t[i] = read_value!(i64);
        d[i] = read_value!(i64);
    }

    let mut p: Vec<usize> = (0..n).collect();
    p.sort_unstable_by(|&pi, &pj| (d[pj] * t[pi]).cmp(&(d[pi] * t[pj])));

    let mut time = 0;
    let mut ans = 0;
    for pi in &p {
        ans += time * d[*pi];
        time += 2 * t[*pi];
    }
    println!("{}", ans);
}
