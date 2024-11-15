use proconio::input;

fn solve(l: usize, r: usize, target: i32, st: &SegmentTree) -> i64 {
    if l == r {
        return (target - st.get(l)) as i64;
    }

    let (max, pos) = st.query_max(l, r);
    let mut ans = (target - max) as i64;
    if pos > l {
        ans += solve(l, pos - 1, max, st);
    }
    if pos < r {
        ans += solve(pos + 1, r, max, st);
    }
    ans
}

struct SegmentTree {
    a: Vec<i32>,
    tree: Vec<(i32, usize)>,
}

impl SegmentTree {
    fn new(a: Vec<i32>) -> Self {
        let n = a.len();
        let tree = vec![(0, 0); 4 * n];
        let mut segment_tree = SegmentTree { a, tree };
        segment_tree.build(1, 0, n - 1);
        segment_tree
    }

    fn build(&mut self, v: usize, start: usize, end: usize) {
        if start == end {
            self.tree[v] = (self.a[start], start);
            return;
        }

        let mid = (start + end) / 2;
        self.build(2 * v, start, mid);
        self.build(2 * v + 1, mid + 1, end);
        self.tree[v] = std::cmp::max(self.tree[2 * v], self.tree[2 * v + 1])
    }

    fn query_max(&self, l: usize, r: usize) -> (i32, usize) {
        self.query(l, r, 1, 0, self.a.len() - 1)
    }

    fn query(&self, l: usize, r: usize, v: usize, start: usize, end: usize) -> (i32, usize) {
        if l == start && r == end {
            return self.tree[v];
        }

        let mid = (start + end) / 2;
        if r <= mid {
            self.query(l, r, 2 * v, start, mid)
        } else if l > mid {
            self.query(l, r, 2 * v + 1, mid + 1, end)
        } else {
            std::cmp::max(
                self.query(l, mid, 2 * v, start, mid),
                self.query(mid + 1, r, 2 * v + 1, mid + 1, end),
            )
        }
    }

    fn get(&self, i: usize) -> i32 {
        return self.a[i];
    }
}

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };
    let seg_tree = SegmentTree::new(a);
    println!("{}", solve(0, n - 1, seg_tree.query_max(0, n - 1).0, &seg_tree));
}
