use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: Usize1,
        y: Usize1,
        grid: [Chars; m],
    }

    let in_bounds = |x, y| x < n && y < m;
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut used = vec![vec![false; n]; m];
    let mut queue = VecDeque::new();
    used[y][x] = true;
    queue.push_back((x, y));
    let mut on_fire = 1;
    while let Some((vx, vy)) = queue.pop_front() {
        for (dx, dy) in &directions {
            let ux = (vx as i32 + dx) as usize;
            let uy = (vy as i32 + dy) as usize;
            if in_bounds(ux, uy) && grid[uy][ux] == '.' && !used[uy][ux] {
                used[uy][ux] = true;
                on_fire += 1;
                queue.push_back((ux, uy));
            }
        }
    }
    println!("{}", on_fire);
}
