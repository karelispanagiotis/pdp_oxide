use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: usize,
        mut faults_per_part: [(usize, i32); c],
    };

    faults_per_part.sort_unstable_by(|(p1, f1), (p2, f2)| f2.cmp(f1).then(p1.cmp(p2)));

    let problematic_parts: Vec<usize> =
        faults_per_part.iter().take_while(|(_, f)| *f != 0).map(|(p, _)| *p).collect();

    println!("{}", problematic_parts.len());
    for part in &problematic_parts {
        println!("{}", part);
    }
}
