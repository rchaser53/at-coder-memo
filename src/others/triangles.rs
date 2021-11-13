#![allow(unused_imports)]
use proconio::{input, fastout};

fn main() {
  input! {
    n: usize,
    mut l: [usize;n]
  }
  l.sort();

  let mut sum = 0;
  for i in 0..n {
    for ii in i+1..n {
      let mut left = 0;
      let mut right = i;
      while left < right {
        let middle = (left + right) / 2;
        if l[ii] < l[i] + l[middle] {
          right = middle;
        } else {
          left = middle + 1;
        }
      }
      sum += i - left;
    }
  }

  println!("{}", sum);
}