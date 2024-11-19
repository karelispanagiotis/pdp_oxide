use proconio::input;

fn main() {
    input! {
        a: i32,
        m: usize,
        y: [i32; m]
    };

    let mut pos1 = a / 2;
    let mut pos2 = a / 2;
    let mut answer1 = 0;
    let mut answer2 = 0;
    for pos in y {
        let new_pos1 = std::cmp::min(a / 2, pos);
        let new_pos2 = std::cmp::max(a / 2, pos);
        answer1 += (pos1 - new_pos1).abs();
        answer2 += (pos2 - new_pos2).abs();
        pos1 = new_pos1;
        pos2 = new_pos2;
    }
    println!("{} {}", answer1, answer2);
}
