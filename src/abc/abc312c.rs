/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;m]
  }

  let mut map = HashMap::new();
  for v in a {
    let entry = map.entry(v).or_insert((0,0));
    entry.0 += 1;
  }
  for v in b {
    let entry = map.entry(v).or_insert((0,0));
    entry.1 += 1;
  }

  let mut que = vec![];
  que.push((1,(0,0)));
  for (v, nums) in map {
    que.push((v,nums));
  }
  que.sort();
  let que = que.into_iter().collect::<VecDeque<_>>();
  
  let mut buy = m;
  let mut sell = 0;
  for (v, (s,b)) in que {
    sell += s;
    if buy <= sell {
      println!("{}", v);
      return
    }

    buy -= b;
    if buy <= sell {
      println!("{}", v+1);
      return
    }
  }
}