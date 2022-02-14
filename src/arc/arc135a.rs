/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 998244353;

struct Helper {
  map:HashMap<usize, usize>
}

impl Helper {
  fn dfs(&mut self, v:usize) -> usize {
    if v <= 4 {
      return v
    }

    if let Some(&nv) = self.map.get(&v) {
      return nv
    }

    let fv = v / 2;
    let mut bv = fv;
    if v % 2 == 1 {
      bv += 1;
    }
    
    let result = self.dfs(fv) * self.dfs(bv) % MOD;
    self.map.insert(v, result);
    result
  }
}

fn main() {
  input! {
    x:usize
  }
  
  let mut helper = Helper { map: HashMap::new() };
  println!("{}", helper.dfs(x));
}