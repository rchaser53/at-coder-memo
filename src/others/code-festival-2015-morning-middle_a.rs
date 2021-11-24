/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    k:usize,
    m:usize,
    r:usize,
    mut vals:[usize;n-1]
  }

  if n == 1 {
    println!("{}", r);
    return
  }

  let need = r * k;
  if k == n {
    let tot = vals.iter().sum::<usize>();
    if need <= tot {
      println!("0");
    } else if tot + m < need {
      println!("-1");
    } else {
      println!("{}", need - tot);
    }
    return
  }

  vals.sort();
  vals.reverse();
  let mut temp = 0usize;
  for i in 0..k {
    temp += vals[i];
  }
  

  if need <= temp {
    println!("0");
    return
  }

  let last_need = need + vals[k-1] - temp;
  if m < last_need {
    println!("-1");
  } else {
    println!("{}", last_need);
  }
}