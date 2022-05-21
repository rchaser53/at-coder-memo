/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut count = 0;
  let mut sum = 0;
  for i in 1..n {
    if s[i] != 'R' { continue }
    
    let mut li = i as i32 -1;
    let mut ri = i as i32 +1;
    let mut temp = 0;
    while 0 <= li && ri < n as i32 {
      if s[li as usize] == 'A' && s[ri as usize] == 'C' {
        temp += 1;
      } else {
        break
      }
      li -= 1;
      ri += 1;
    }
    if 0 < temp {
      sum += temp;
      count += 1;
    }
  }

  println!("{}", std::cmp::min(sum, count * 2));
}