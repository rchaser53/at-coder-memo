/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    m:usize,
    rows:[Chars;3]
  }

  let p = "0123456789".chars().collect::<Vec<char>>();
  let mut memo = vec![true;10];
  for &c in &p {
    for i in 0..3 {
      let mut exist = false;
      for j in 0..m {
        if rows[i][j] == c {
          exist = true;
          break
        }
      }
      if !exist {
        memo[c as usize - '0' as usize] = false;
        break
      }
    }
  }

  let default = 1000000;
  let mut result = default;
  let seq = [[0,1,2],[0,2,1],[1,0,2],[1,2,0],[2,0,1],[2,1,0]];
  for i in 0..10 {
    if !memo[i] { continue }
    let c = p[i];
    for arr in &seq {
      let mut temp = 0;
      for &j in arr {
        while rows[j][temp % m] != c {
          temp += 1;
        }
        temp += 1;
      }
      temp -= 1;
      result = result.min(temp);
    }
  }

  if result == default {
    println!("-1");
  } else {
    println!("{}", result);
  }
}
