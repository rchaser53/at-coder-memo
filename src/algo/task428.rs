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
  
  fn helper(n:usize,l:usize,r:usize) -> Vec<VecDeque<usize>> {
    if r < l {
      return vec![]
    }
    if n == 0 {
      return vec![VecDeque::new()]
    }

    let mut result = vec![];
    let temp = helper(n-1,l,r);
    for mut v in temp {
      v.push_front(l);
      result.push(v);
    }

    let temp = helper(n,l+1,r);
    for v in temp {
      result.push(v);
    }

    result
  }
  
  fn main() {
    let vals: Vec<usize> = readvec();
    let (n,l,r) = (vals[0], vals[1], vals[2]);

    let result = helper(n,l,r);
    for v in result {
      println!("{}", v.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
    }
  }