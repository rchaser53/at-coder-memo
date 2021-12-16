/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
  use std::cmp::Reverse;
  
  fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
  }
  
  fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
  }
  
  fn helper(memo: &mut Vec<Option<bool>>, vals: &Vec<usize>, i:usize, j:usize) -> bool {

    if let Some(v) = memo[j] {
      return v;
    }

    if i == 0 {
      memo[j] = Some(j == 0);
      return j == 0
    }
    
    let mut flag = false;
    if j >= vals[i-1] && helper(memo, vals, i-1, j-vals[i-1]) {
      memo[j-vals[i-1]] = Some(true);
      flag = true;
    }
    if helper(memo, vals, i-1, j) {
      memo[j] = Some(true);
      flag = true;
    }
    flag
  }
  
  fn main() {
    let vals: Vec<usize> = readvec();
    let (n,x) = (vals[0], vals[1]);
    let vals: Vec<usize> = readvec();
    let mut memo = vec![None;10000+1];
    memo[0] = Some(true);
  
    if helper(&mut memo, &vals, n, x) {
      println!("Yes");
    } else {
      println!("No");
    }
  }