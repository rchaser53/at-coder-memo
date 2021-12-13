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
  let temp:Vec<usize> = readvec();
  // let (n,m) = (temp[0], temp[1]);
  let mut a:Vec<usize> = readvec();
  let b = readvec();
  let mut b = b.into_iter().collect::<VecDeque<usize>>();
  
  a.sort();
  
  let mut result = 0;
  for av in a {
    while let Some(bv) = b.pop_front() {
      if av <= bv {
        result += 1;
        break
      }
    }
  }
  println!("{}", result);
}