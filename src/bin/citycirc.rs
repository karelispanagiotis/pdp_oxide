use proconio::{input, read_value};

fn main() {
    input! { n: usize };
    let mut b = vec![0; 2 * n];
    for i in 0..n {
        b[i] = read_value! { i64 };
        b[i + n] = b[i];
    }

    let mut ans = 0;
    let mut i = 0;
    while i < n && ans == 0 {
        let mut j = i;
        let mut sum = b[j];
        while sum >= 0 && j - i + 1 < n {
            j += 1;
            sum += b[j];
        }
        if sum >= 0 && j - i + 1 == n {
            ans = i + 1;
        }
        i = j + 1;
    }
    println!("{}", ans);
}
