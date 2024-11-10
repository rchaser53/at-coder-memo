/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    q:usize
  }

  let mut temp = 0;
  let mut que = VecDeque::new();
  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      que.push_back((1,true));
    } else if t == 2 {
      input! {
        v:isize
      }
      temp += v;
      que.push_back((v,false));
    } else {
      input! {
        border:isize
      }

      let mut count = 0;
      while let Some((v,f)) = que.pop_front() {
        if f {
          if border <= temp {
            count += 1;
          } else {
            que.push_front((v,f));
            break
          }
        } else {
          temp -= v;
        }
      }
      println!("{}", count);
    }
  }
}