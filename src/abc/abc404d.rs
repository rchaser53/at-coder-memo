/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::collections::*;

struct Helper {
  n:usize,
  m:usize,
  memo:Vec<Vec<usize>>,
  c:Vec<usize>,
  result:usize
}

impl Helper {
  fn dfs(&mut self, i:usize, arr: &mut Vec<usize>) {
    if i == self.n {
      let mut count = vec![0;self.m];
      let mut temp = 0;

      for j in 0..self.n {
        let v = arr[j];
        for k in &self.memo[j] {
          count[*k] += v;
        }

        temp += self.c[j] * v;
      }

      let mut flag = true;
      for j in 0..self.m {
        if count[j] < 2 {
          flag = false;
          break;
        }
      }

      if flag {
        self.result = self.result.min(temp);
      }

      return;
    }

    for j in 0..3 {
      arr.push(j);
      self.dfs(i+1, arr);
      arr.pop();
    }
  }
}

fn main() {
  input!{
    n:usize,
    m:usize,
    c:[usize;n],
  }

  let mut memo = vec![vec![];n];
  for i in 0..m {
    input! {
      k:usize,
      a:[Usize1;k]
    }
    for j in a {
      memo[j].push(i);
    }
  }

  let mut helper = Helper {
    n,
    m,
    memo,
    c,
    result: usize::MAX
  };
  helper.dfs(0, &mut vec![]);

  println!("{}", helper.result);
}
