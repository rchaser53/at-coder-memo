/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn dfs(memo: &Vec<Vec<usize>>, ci:usize, level:usize, want_value:usize) -> usize {
  if level == 0 {
    return if want_value == memo[0][ci] {
      0
    } else {
      1
    }
  }

  let mut temps = vec![0;3];
  for i in 0..3 {
    let ni = ci*3+i;
    let nv = memo[level-1][ni];
    let cost = if nv == want_value {
      0
    } else {
      dfs(&memo, ni, level-1, want_value)
    };
    temps[i] = cost;
  }
  temps.sort();

  temps[0] + temps[1]
}

fn main() {
  input! {
    n:usize,
    a:Chars
  }

  let a = a.into_iter().map(|c| c as usize - '0' as usize).collect::<Vec<usize>>();
  let mut memo = vec![a];

  while memo[memo.len()-1].len() > 1 {
    let mut new_memo = vec![];
    let li = memo.len() - 1;
    let m = memo[li].len();

    for i in 0..m/3 {
      let v = memo[li][3*i] + memo[li][3*i+1] + memo[li][3*i+2];
      if 1 < v {
        new_memo.push(1);
      } else {
        new_memo.push(0);
      }
    }
    memo.push(new_memo);
  }

  let li = memo.len()-1;
  if memo[li][0] == 1 {
    println!("{}", dfs(&memo, 0, li, 0));
  } else {
    println!("{}", dfs(&memo, 0, li, 1));
  };
}