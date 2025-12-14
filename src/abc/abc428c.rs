/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    q:usize
  }

  let mut out = None;
  let mut stack = vec![];
  let mut status = (0,0);
  for _ in 0..q {
    input! {
      t:i32,
    }

    if t == 1 {
      input! {
        c:char,
      }

      stack.push(c);
      if c == '(' {
        status.0 += 1;
      } else {
        status.1 += 1;
      }

      if let Some(_) = out {
        println!("No");
      } else {
        if status.0 < status.1 {
          out = Some(stack.len()-1);
          println!("No");
        } else if status.0 == status.1 {
          println!("Yes");
        } else {
          println!("No");
        }
      }
    } else {
      let li = stack.len()-1;
      let c = stack.pop().unwrap();
      if c == '(' {
        status.0 -= 1;
      } else {
        status.1 -= 1;
      }

      if let Some(ti) = out {
        if li == ti {
          out = None;
        } else {
          println!("No");
          continue
        }
      }

      if status.0 == status.1 {
        println!("Yes");
      } else {
        println!("No");
      }
    }
  }
}