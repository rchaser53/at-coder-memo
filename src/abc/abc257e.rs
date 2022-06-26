/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
  use proconio::marker::*;
  use std::collections::*;
  use std::cmp::*;
  
  fn helper(a:&(Vec<usize>, usize), b:&(Vec<usize>, usize)) -> bool {
    if a.1 < b.1 {
      true
    } else if a.1 == b.1 {
      for j in (0..9).rev() {
        if a.0[j] < b.0[j] {
          return true
        } else if a.0[j] > b.0[j] {
          return false
        }
      }
      false
    } else {
      false
    }
  }
  
  fn main() {
    input! {
      n:usize,
      mut vals:[usize;9]
    }
  
    let mut memo = vec![(vec![0;9],0);n+1];
    let mut max = (vec![0;9],0);
    for i in 0..n {
      for j in 0..9 {
        let ni = i + vals[j];
        if n < ni { continue }
  
        memo[i].0[j] += 1;
        memo[i].1 += 1;
        if helper(&memo[ni], &memo[i]) {
          if helper(&max, &memo[i]) {
            max = memo[i].clone();
          }
          memo[ni] = memo[i].clone();
        }
        
        memo[i].0[j] -= 1;
        memo[i].1 -= 1;
      }
    }
      
    let mut result = vec![];
    for i in (0..9).rev() {
      for _ in 0..max.0[i] {
        result.push(i+1);
      }
    }
    println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(""));
  }