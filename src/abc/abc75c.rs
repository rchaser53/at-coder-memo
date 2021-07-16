/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
    input! {
      n:usize,
      m:usize,
      edges:[(Usize1,Usize1);m]
    }

    let mut result = 0;
    for i in 0..m {
      let mut tree: UnionFind<usize> = UnionFind::new(n);
      for j in 0..m {
        if i == j { continue }
        let (from, to) = edges[j];
        tree.union(from, to);
        tree.union(to, from);
      }

      let last = tree.find(0);
      for i in 1..n {
        if tree.find(i) != last {
          result += 1;
          break
        }
      }
    }
    println!("{}", result);
}
