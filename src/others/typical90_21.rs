/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      m:usize,
      vals:[(Usize1, Usize1);m]
    }

    let mut neighbors = vec![vec![]; n];
    for (from, to) in vals {
        neighbors[from].push(to);
    }
    let result = scc(&neighbors);
    let mut map = HashMap::new();
    for v in result {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut result = 0usize;
    for (_, v) in map {
        result += v * (v - 1) / 2;
    }
    println!("{}", result);
}

pub fn scc(g: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = g.len();

    // Post-order traversal
    let mut po = vec![];
    {
        fn dfs(u: usize, g: &Vec<Vec<usize>>, mut used: &mut Vec<bool>, mut po: &mut Vec<usize>) {
            if used[u] {
                return;
            }
            used[u] = true;
            for &v in g[u].iter() {
                if !used[v] {
                    dfs(v, &g, &mut used, &mut po);
                }
            }
            po.push(u);
        }
        let mut used = vec![false; n];
        for u in 0..n {
            dfs(u, &g, &mut used, &mut po);
        }
    }

    let mut g_r = vec![vec![]; n];
    for u in 0..n {
        for &v in g[u].iter() {
            g_r[v].push(u);
        }
    }

    // Components
    let mut cmp = vec![0; n];
    {
        let mut used = vec![false; n];
        let mut k = 0;
        po.reverse();
        for &u in po.iter() {
            let mut stack = vec![u];
            if used[u] {
                continue;
            }
            while let Some(v) = stack.pop() {
                if used[v] {
                    continue;
                }
                used[v] = true;
                cmp[v] = k;
                for &w in g_r[v].iter() {
                    stack.push(w)
                }
            }
            k += 1;
        }
    }

    cmp
}
