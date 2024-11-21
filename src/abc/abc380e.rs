/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

struct UnionFind {
  parents: Vec<usize>,
  sizes: Vec<usize>,
  ranks: Vec<usize>,
  neighbors: Vec<(isize,isize)>,
  colors: Vec<usize>
}

impl UnionFind {
  fn new(n:usize) -> UnionFind {
    let mut uf = UnionFind { parents: vec![0;n], sizes:vec![1;n], ranks:vec![0;n], neighbors: vec![(0,0);n], colors: vec![0;n] };
    for i in 0..n {
      uf.parents[i] = i;
      uf.neighbors[i] = (i as isize - 1, i as isize + 1);
      uf.colors[i] = i;  
    }
    uf
  }

  fn root(&mut self, x:usize) -> usize {
    let ti = self.parents[x];
    if ti == x {
      x
    } else {
      self.parents[x] = self.root(ti);
      self.parents[x]
    }
  }

  fn is_same(&mut self, x:usize, y:usize) -> bool {
    self.root(x) == self.root(y)
  }

  fn unite(&mut self, x:usize, y:usize) -> bool {
    let rx = self.root(x);
    let ry = self.root(y);
    if rx == ry { return false }
    let (rx, ry) = if self.ranks[rx] < self.ranks[ry] {
      (ry, rx)
    } else {
      (rx, ry)
    };
    self.parents[ry] = rx;
    if self.ranks[rx] == self.ranks[ry] {
      self.ranks[rx] += 1;
    }

    let li = self.neighbors[rx].0.min(self.neighbors[ry].0);
    let ri = self.neighbors[rx].1.max(self.neighbors[ry].1);
    self.neighbors[rx].0 = li;
    self.neighbors[ry].0 = li;

    self.neighbors[rx].1 = ri;
    self.neighbors[ry].1 = ri;

    self.colors[ry] = self.colors[rx];

    self.sizes[rx] += self.sizes[ry];
    true
  }

  fn group_size(&mut self, x:usize) -> usize {
    let i = self.root(x);
    self.sizes[i]
  }
}

fn main() {
  input! {
    n:usize,
    q:usize,
  }

  let mut utree = UnionFind::new(n);
  let mut map = HashMap::new();
  for i in 0..n {
    map.insert(i, 1);
  }

  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        x: Usize1,
        c: Usize1
      }

      let pi = utree.root(x);
      let cc = utree.colors[pi];

      if cc == c {
        continue
      }

      utree.colors[pi] = c;

      // 現状のカラーからサイズの分を引く
      let size = utree.group_size(pi);
      *map.entry(cc).or_insert(0) -= size;
      *map.entry(c).or_insert(0) += size;

      let (li, ri) = utree.neighbors[pi];
      if li > -1 {
        let li = li as usize;
        let lpi = utree.root(li);
        let lc = utree.colors[lpi];

        if lc == c {
          utree.unite(lpi, pi);
        }
      }

      if ri < n as isize {
        let ri = ri as usize;
        let rpi = utree.root(ri);
        let rc = utree.colors[rpi];
        
        if rc == c {
          utree.unite(rpi, pi);
        }
      }

      continue
    }

    input! {
      c: Usize1
    }

    println!("{}", map.get(&c).unwrap_or(&0));
  }
}