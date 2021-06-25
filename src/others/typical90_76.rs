/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let total = vals.iter().sum::<usize>();
  if total % 10 != 0 {
    println!("No");
    return
  }
  let goal = total / 10;

  let mut left = 0;
  let mut right = 0;
  let mut temp = vals[0];
  let right_limit = 2 * n;
  while left < n {
    if temp == goal {
      println!("Yes");
      return
    } else if temp < goal {
      if right_limit <= right {
        println!("No");
        return
      } else {
        right += 1;
        temp += vals[right % n];
      }
    } else {
      if left == right {
        left += 1;
        right += 1;
        if n <= left {
          println!("No");
          return
        }
        temp = vals[right % n];
      } else {
        temp -= vals[left % n];
        left += 1;
      }
    }
  }
  
  if temp == goal {
    println!("Yes");
  } else {
    println!("No");
  }
}
