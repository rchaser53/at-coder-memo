/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
    pt:[(usize,usize);n-1],
    q:usize,
    queries:[usize;q]
  }

  let type_num = 840; // lcm(1,2,3,4,5,6,7,8)
  let mut memo = vec![0;type_num];
  for i in 0..type_num {
    let start = if i % pt[0].0 == 0 {
      i
    } else {
      i + pt[0].0 - (i % pt[0].0)
    };
    let mut now = start + pt[0].1;
    for j in 1..n-1 {
      let (p,t) = pt[j];
      if now % p != 0 {
        now += p - (now % p);
      }
      now += t;
    }
    memo[i] = now+y-i;
  }

  let mut tool = vec![];
  for i in 0..=type_num {
    tool.push(i*pt[0].0);
  }
  for s in queries {
    let to_n1 = s+x;
    println!("{}", to_n1 + memo[to_n1 % type_num]);
  }

}