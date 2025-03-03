use proconio::input;

fn dfs(
    v: usize,
    adj: &[Vec<usize>],
    genders: &[char],
    males_under: &mut [i32],
    females_under: &mut [i32],
) -> i64 {
    let mut result = 0;
    for u in &adj[v] {
        result += dfs(*u, adj, genders, males_under, females_under);
        males_under[v] += males_under[*u];
        females_under[v] += females_under[*u];
    }

    if genders[v] == 'm' {
        males_under[v] += 1;
        result + females_under[v] as i64
    } else {
        females_under[v] += 1;
        result - males_under[v] as i64
    }
}

fn main() {
    input! {
        n: usize,
        pairs: [(usize, char); n],
    };

    let mut adj = vec![Vec::new(); n + 1];
    let mut genders = vec!['u'; n + 1];
    let mut root = 0;
    for (i, (parent, gender)) in (1..=n).zip(pairs) {
        adj[parent].push(i);
        genders[i] = gender;
        if parent == 0 {
            root = i;
        }
    }

    let mut males_under = vec![0; n + 1];
    let mut females_under = vec![0; n + 1];
    println!("{}", dfs(root, &adj, &genders, &mut males_under, &mut females_under));
}
