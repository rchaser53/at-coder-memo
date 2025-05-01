/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;


fn main() {
  input! {
    n:usize,
    d:usize,
    a:[usize;n]
  }

  let max = *a.iter().max().unwrap();
  let map = a.iter().map(|&x| (x, 1usize)).into_grouping_map().sum();

  if d == 0 {
    println!("{}", n - map.len());
    return
  }

  println!("{}",
    (0..d)
      .map(|r| {
        (r..=max)
          .step_by(d)
          .map(|x| *map.get(&x).unwrap_or(&0))
          .tuple_windows()
          .fold((0,0), |(a,b), (x,y)| (b, (a+x).min(b+y)))
          .1
      })
      .sum::<usize>()
  );
}