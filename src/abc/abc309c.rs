/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:isize,
    ab:[(isize,isize);n]
  }

  let mut tot = 0;
  for i in 0..n {
    tot += ab[i].1;
  }

  if tot <= k {
    println!("1");
    return
  }

  let mut map = HashMap::new();
  for (a,b) in ab {
    *map.entry(a).or_insert(0) += b;
  }

  let mut ms = map.into_iter().collect::<Vec<(isize,isize)>>();
  ms.sort();

  for &(a,b) in &ms {
    tot -= b;
    if tot <= k {
      println!("{}", a+1);
      return
    }
  }
}