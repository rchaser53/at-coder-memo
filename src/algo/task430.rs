/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
  use std::cmp::Reverse;
  
  fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
  }
  
  fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
  }

  fn create_pascal() -> Vec<Vec<usize>> {
    let n = 100;
    let mut result: Vec<Vec<usize>> = vec![vec![0;n+1];n+1];
    result[0][0] = 1;
    for i in 0..n {
      for ii in 0..n {
        result[i+1][ii] = result[i+1][ii] + result[i][ii];
        result[i+1][ii+1] = result[i+1][ii+1] + result[i][ii];
      } 
    }
    result
  }
  
  fn main() {
    let vals: Vec<usize> = readvec();
    let (n,l,r) = (vals[0], vals[1], vals[2]);

    let v = r - l + 1;
    if v < n {
      println!("0");
    } else {
      let pascal = create_pascal();
      println!("{}", pascal[v][n]);
    }
  }