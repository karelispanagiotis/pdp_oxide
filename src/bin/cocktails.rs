use proconio::input;

struct Graph {
    adj: Vec<Vec<(usize, i32, i32)>>,
}

impl Graph {
    fn calculate_coeffs(&self) -> Vec<f64> {
        let mut coeffs = vec![1.0; self.adj.len()];
        self.dfs(0, 0, &mut coeffs);
        coeffs
    }

    fn dfs(&self, v: usize, par: usize, coeffs: &mut [f64]) {
        for edge in &self.adj[v] {
            let (u, p, q) = *edge;
            if u != par {
                coeffs[u] = q as f64 / p as f64 * coeffs[v];
                self.dfs(u, v, coeffs);
            }
        }
    }
}

fn main() {
    input! { n: usize };
    let mut adj = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! { a: usize, b: usize, p: i32, q: i32 };
        adj[a].push((b, p, q));
        adj[b].push((a, q, p));
    }

    let graph = Graph { adj };
    let coeffs = graph.calculate_coeffs();
    let percentage_0 = 1.0 / coeffs.iter().sum::<f64>();
    for coeff in &coeffs {
        println!("{:.2}", *coeff * percentage_0 * 100.0);
    }
}
