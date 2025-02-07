/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    w:usize,
    mut xy:[(Usize1,usize);n],
    q:usize,
    ta:[(usize,Usize1);q]
  }

  let mut memo = vec![vec![];w];
  for i in 0..n {
    let (x,y) = xy[i];
    memo[x].push((y,i));
  }

  let inf = 10usize.pow(15);
  let mut min = inf;
  for i in 0..w {
    memo[i].sort();
    min = min.min(memo[i].len());
  }

  let mut result = vec![inf;n];
  for i in 0..min {
    let mut max = 0;
    for j in 0..w {
      max = max.max(memo[j][i].0);
    }

    for j in 0..w {
      let ti = memo[j][i].1;
      result[ti] = max;
    }
  }
  
  for (t,a) in ta {
    if result[a] <= t {
      println!("No");
    } else {
      println!("Yes");
    }
  }
}