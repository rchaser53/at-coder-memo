/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize,
    q:usize,
    vals:[(usize,Usize1,Usize1,isize);q]
  }

  let mut tree: UnionFind<usize> = UnionFind::new(n);

  let mut outs = vec![false;q];
  let mut vs = vec![0;n-1];
  for i in 0..q {
    let (t, l, r, v) = vals[i];
    if t == 0 {
      tree.union(l, r);
      tree.union(r, l);
      vs[l] = v;
    } else {
      if tree.find(l) != tree.find(r) {
        outs[i] = true;
      }
    }
  }

  let inf = 1_000_000_000_000_000isize;
  let mut base = vec![inf;n];
  base[0] = 0;

  for i in 0..n-1 {
    let v = vs[i];
    base[i+1] = v - base[i];
  }

  for i in 0..q {
    let (t, l, r, v) = vals[i];
    if t == 1 {
      if outs[i] {
        println!("Ambiguous");
      } else {
        let diff = v - base[l];
        if l % 2 == r % 2 {
          println!("{}", base[r] + diff);
        } else {
          println!("{}", base[r] - diff);
        }
      }
    }
  }
}
