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

fn helper(memo: &mut Vec<Option<usize>>, x:usize) -> usize {
  if let Some(v) = memo[x] {
    v
  } else {
    let v = helper(memo, x - 1) + x;
    memo[x] = Some(v);
    v
  }
}

fn main() {
  let ab: Vec<usize> = readvec();
  let (a,b) = (ab[0], ab[1]);
  
  let mut memo = vec![None;b+1];
  memo[0] = Some(0);

  let bv = helper(&mut memo, b);
  let av = helper(&mut memo, a-1);

  println!("{}", bv - av);
}