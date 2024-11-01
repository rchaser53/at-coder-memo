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
  ranks: Vec<usize>
}

impl UnionFind {
  fn new(n:usize) -> UnionFind {
    let mut uf = UnionFind { parents: vec![0;n], sizes:vec![1;n], ranks:vec![0;n] };
    for i in 0..n {
      uf.parents[i] = i;
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
    queries:[(usize,Usize1,Usize1);q]
  }

  let mut memo = vec![BTreeSet::<usize>::new();n];
  for i in 0..n {
    memo[i].insert(i);
  }

  let mut utree = UnionFind::new(n);
  for (t,a,b) in queries {
    if t == 1 {
      let pai = utree.root(a);
      let pbi = utree.root(b);
      if utree.unite(a,b) {
        let pnai = utree.root(a);
        let pnbi = utree.root(b);

        if pai != pnai {
          let mut x = BTreeSet::new();
          std::mem::swap(&mut memo[pai], &mut x);
          for v in x {
            memo[pnai].insert(v);
          }
        }
        
        if pbi != pnbi {
          let mut x = BTreeSet::new();
          std::mem::swap(&mut memo[pbi], &mut x);
          for v in x {
            memo[pnbi].insert(v);
          }          
        }
      }
    } else {
      let pi = utree.root(a);
      if memo[pi].len() <= b {
        println!("-1");
      } else {
        let mut count = 0;
        for &v in memo[pi].iter().rev() {
          if count == b {
            println!("{}", v+1);
            break
          }
          count += 1;
        }
      }
    }
  }
}