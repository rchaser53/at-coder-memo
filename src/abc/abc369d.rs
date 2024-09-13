/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  if n <= 2 {
    let mut result = 0;
    for i in 0..n {
      result += a[i] * (i+1);
    }
    println!("{}", result);
    return
  }

  // .0 x2の値が入る
  // .1 x1の値が入る
  let mut memo = vec![(0,0);n];
  memo[0].0 = 0;
  memo[0].1 = a[0];
  memo[1].0 = a[0]+a[1]*2;
  memo[1].1 = a[0].max(a[1]);
  
  for i in 2..n {
    let p_x2 = memo[i-2].0; // 最後にx2の値が入った
    let p_x1 = memo[i-2].1; // 最後にx1の値が入った
    let a1 = a[i-1];
    let a2 = a[i];
    
    let x2_arr = vec![p_x2+a1+a2*2, p_x1+a1*2, p_x1+a2*2];
    let x1_arr = vec![p_x1+a1*2+a2, p_x2+a1, p_x2+a2];

    memo[i].0 = *x2_arr.iter().max().unwrap();
    memo[i].1 = *x1_arr.iter().max().unwrap();
  }

  println!("{}", memo[n-1].0.max(memo[n-1].1));
}