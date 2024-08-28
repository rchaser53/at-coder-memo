/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

struct Helper {
  set: HashSet<usize>,
  v: HashSet<usize>
}

impl Helper {
  fn dfs(&mut self, ci:usize, li:usize, g: &Vec<Vec<usize>>, memo: &mut Vec<usize>) {
    for &ni in &g[ci] {
      if ni == li { continue }

      memo.push(ni);
      if self.v.contains(&ni) && !self.set.contains(&ni) {
        for j in memo.iter().rev() {
          if self.set.contains(&j) { break }
          self.set.insert(*j);
        }
      }

      self.dfs(ni, ci, g, memo);
      memo.pop();
    }
  }
}

fn main() {
  input! {
    n:usize,
    k:usize,
    ab:[(Usize1,Usize1);n-1],
    v:[Usize1;k]
  }

  let mut g = vec![vec![];n];
  for (a,b) in ab {
    g[a].push(b);
    g[b].push(a);
  }

  let bi = v[0];
  let mut set = HashSet::new();
  set.insert(bi);

  let mut helper = Helper {
    set, v: v.into_iter().collect::<HashSet<usize>>()
  };
  
  helper.dfs(bi, 1_000_000_000, &g, &mut vec![bi]);  
  println!("{}", helper.set.len());

}