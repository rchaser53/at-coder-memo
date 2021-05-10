/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
) {
    input! {
      t:isize,
      n:usize,
    }

    let mut memo = vec![];
    let mut temp = 0isize;
    loop {
      let mut count = 0;
      while temp < 100 {
        temp += t;
        count += 1;
      }
      memo.push(count);
      temp %= 100;
      if temp == 0 {
        break
      }
    }

    if n == 1 {
      println!("{}", memo[0]);
    } else if n <= memo.len() {
      let mut temp = -1;
      for i in 0..n {
        temp += memo[i];
      }
      println!("{}", temp);
    } else {
      let tot = memo.iter().sum::<isize>();
      let len = memo.len() as isize;
      let mut temp = tot + len as isize - 1;
      let rn = n as isize - len;
      let remainder = rn % len;

      temp += (rn / len) * (tot + len);
      for i in 0..remainder {
        temp += memo[i as usize] + 1;
      }
      println!("{}", temp);
    }

}
