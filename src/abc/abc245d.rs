/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[isize;n+1],
    mut c:[isize;n+m+1]
  }

  let mut result = vec![];
  for i in (0..=m).rev() {
    for j in -10000..=10000 {
      if c[i+n] == a[n] * j {
        result.push(j);
        for k in 0..=n {
          c[i+k] -= a[k] * j;
        }
        break
      }
    }
  }
  result.reverse();
  
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}