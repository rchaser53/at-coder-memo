/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }

  let mut memo = vec![0;26];
  let mut stack = vec![];
  for c in s {
    if c == '(' {
      stack.push(c);
    } else if c == ')' {
      while let Some(cc) = stack.pop() {
        if cc == '(' {
          break
        }
        let ti = (cc as u8 - 'a' as u8) as usize;
        memo[ti] -= 1;
      }
    } else {
      let ti = (c as u8 - 'a' as u8) as usize;
      memo[ti] += 1;
      if 1 < memo[ti] {
        println!("No");
        return
      }
      stack.push(c);
    }
  }
  println!("Yes");
}