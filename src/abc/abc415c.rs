/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    t:usize
  }

  for _ in 0..t {
    input! {
      n:usize,
      s:Chars
    }

    let limit = 1 << n;
    let mut memo = vec![false;limit];

    if s[limit-2] == '1' {
      println!("No");
      continue
    }

    for x in 0..limit-1 {
      if s[x] == '1' {
        memo[x+1] = true;
      }
    }

    let mut success = false;
    let mut i = 0;
    while !success && i < n {

      let si = 1 << i;
      if memo[si] {
        i += 1;
        continue;
      }

      memo[si] = true;
      let mut stack = vec![si];

      while !stack.is_empty() {
        let mut new_stack = vec![];
        for j in stack {
          for x in 0..n {
            if j >> x & 1 == 0 {
              let next = j | (1 << x);
              if !memo[next] {
                memo[next] = true;
                new_stack.push(next);
              }
            }
          }
        }
        stack = new_stack;
      }
      if memo[limit - 1] {
        success = true;
        break
      } else {
        i += 1;
      }
    }

    if success {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}