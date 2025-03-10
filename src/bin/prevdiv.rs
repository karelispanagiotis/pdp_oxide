use proconio::input;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn main() {
    input! { n: usize, nums: [i64; n] };
    let mut cur_lcm = nums[0];
    let mut ans = nums[0];
    for num in &nums {
        if *num % cur_lcm == 0 {
            ans = ans.max(*num);
        }
        cur_lcm = lcm(cur_lcm, *num);
    }
    println!("{}", ans);
}
