/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize
  }

  let mut result = vec![vec!['.';n];n];
  for i in 0..n {
    for j in 0..3 {
      let rj = (3*i+j) % n;
      result[i][rj] = '#';
    }
  }

  if n % 3 != 0 {
    let v = n / 3;
    result.swap(0, v-1);
    result.swap(n-1, n-v);
  }
  
  for row in result {
    println!("{}", row.iter().map(|v| v.to_string()).collect::<String>());
  }
}