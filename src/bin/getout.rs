use proconio::input;
use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

#[derive(Eq, Hash, PartialEq, Clone)]
struct ParkingLot {
    n: usize,
    rows: [[bool; 10]; 10],
    cols: [[bool; 10]; 10],
}

impl ParkingLot {
    fn is_solution(&self, r: usize) -> bool {
        for i in 1..=self.n {
            if self.rows[r][i] {
                return false;
            }
        }
        for i in 1..=self.n {
            if self.cols[i][r] || self.cols[i][r - 1] {
                return false;
            }
        }
        true
    }

    fn get_possible_transitions(&self) -> Vec<Self> {
        let mut transitions = Vec::new();
        let matrix = self.as_matrix();
        for i in 1..=self.n {
            for j in 1..=self.n {
                if self.rows[i][j] {
                    for pos in self.get_free_range_row(&matrix, j, i) {
                        let mut next_state = self.clone();
                        next_state.rows[i][j] = false;
                        next_state.rows[i][pos] = true;
                        transitions.push(next_state);
                    }
                }

                if self.cols[i][j] {
                    for pos in self.get_free_range_col(&matrix, j, i) {
                        let mut next_state = self.clone();
                        next_state.cols[i][j] = false;
                        next_state.cols[i][pos] = true;
                        transitions.push(next_state);
                    }
                }
            }
        }
        transitions
    }

    fn as_matrix(&self) -> [[bool; 10]; 10] {
        let mut matrix = [[false; 10]; 10];
        for i in 1..=self.n {
            for j in 1..=self.n {
                if self.rows[i][j] {
                    matrix[i][j] = true;
                    matrix[i][j + 1] = true;
                }

                if self.cols[i][j] {
                    matrix[j][i] = true;
                    matrix[j + 1][i] = true;
                }
            }
        }
        matrix
    }

    fn get_free_range_row(
        &self,
        matrix: &[[bool; 10]; 10],
        pos: usize,
        row: usize,
    ) -> Range<usize> {
        let mut l = pos;
        while l > 1 && !matrix[row][l - 1] {
            l -= 1;
        }
        let mut r = pos + 1;
        while r < self.n && !matrix[row][r + 1] {
            r += 1;
        }
        l..r
    }

    fn get_free_range_col(
        &self,
        matrix: &[[bool; 10]; 10],
        pos: usize,
        col: usize,
    ) -> Range<usize> {
        let mut d = pos;
        while d > 1 && !matrix[d - 1][col] {
            d -= 1;
        }
        let mut u = pos + 1;
        while u < self.n && !matrix[u + 1][col] {
            u += 1;
        }
        d..u
    }
}

fn bfs(source: ParkingLot, r: usize) -> i32 {
    if source.is_solution(r) {
        return 0;
    }

    let mut queue = VecDeque::new();
    let mut used = HashMap::new();
    let mut dist = HashMap::new();

    used.insert(source.clone(), false);
    dist.insert(source.clone(), 0);
    queue.push_back(source);

    while let Some(curr_lot) = queue.pop_front() {
        let curr_dist = *dist.get(&curr_lot).unwrap();
        for next_lot in curr_lot.get_possible_transitions() {
            let used_next = used.entry(next_lot.clone()).or_default();
            let dist_next = dist.entry(next_lot.clone()).or_default();
            if !*used_next {
                *used_next = true;
                *dist_next = curr_dist + 1;
                queue.push_back(next_lot.clone());
                if next_lot.is_solution(r) {
                    return *dist_next;
                }
            }
        }
    }
    -1
}

fn main() {
    input! { n: usize, r: usize };

    let mut source = ParkingLot { n, rows: [[false; 10]; 10], cols: [[false; 10]; 10] };
    for i in 1..=n {
        input! { x: usize, coords: [usize; x] };
        for coord in coords {
            source.rows[i][coord] = true;
        }
    }
    for i in 1..=n {
        input! { x: usize, coords: [usize; x] };
        for coord in coords {
            source.cols[i][coord] = true;
        }
    }

    println!("{}", bfs(source, r));
}
