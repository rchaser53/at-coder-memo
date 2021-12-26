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

fn main() {
  let dict:Vec<usize> = readvec();
  let (n, w) = (dict[0], dict[1]);
  let mut vals = vec![];
  for _ in 0..n {
    let dict:Vec<usize> = readvec();
    vals.push((dict[0], dict[1] as isize));
  }
  
  let mut memo = vec![-1;w+1];
  memo[0] = 0;
  for (cw,cv) in vals {
    let mut new_memo = memo.clone();
    for i in 0..w {
      if memo[i] == -1 { continue }
      let nw = i+cw;
      if w < nw { continue }
      new_memo[nw] = std::cmp::max(new_memo[nw], memo[i] + cv);
    }
    memo = new_memo;
  }

  let mut max = 0isize;
  for v in memo {
    max = std::cmp::max(max, v);
  }

  println!("{}", max);
}