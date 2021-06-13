/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

struct Helper {
  set: HashSet<usize>,
  total: usize,
  g: Vec<Vec<usize>>,
  flag: bool
}

impl Helper {
  pub fn culc(
    &mut self,
    last: usize,
    ci: usize
  ) {
    if self.set.contains(&ci) {
      self.flag = false;
      return
    }
    self.set.insert(ci);

    for i in 0..self.g[ci].len() {
      let to = self.g[ci][i];
      if to == last { continue }
      self.culc(ci, to);
    }
  }
}


pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m],
  }

  let mut g = vec![vec![];n];

  for (from, to) in edges {
    g[from].push(to);
    g[to].push(from);
  }

  let mut helper = Helper{
    set:HashSet::new(), total:0, g, flag:true
  };

  for i in 0..n {
    helper.flag = true;
    helper.culc(1_000_000_000, i);
    if helper.flag {
      helper.total += 1;
    }
  }

  println!("{}", helper.total);
}