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
    n:usize,
    s:Chars
  }

  let mut memo = (vec![0usize;n]).into_iter().collect::<VecDeque<usize>>();
  let mut temp = 0usize;
  for i in 0..n {
    let num = i + 1;
    let v = s[i] as usize - '0' as usize;
    temp += v * num;
    memo[i] += temp;
  }

  for _ in 0..20 {
    memo.push_front(0);
  }

  let m = memo.len();
  for i in (0..m).rev() {
    let mut num = memo[i];
    memo[i] = 0;
    let mut x = 0;
    while 0 < num {
      memo[i-x] += num % 10;
      num /= 10;
      x += 1;
    }
  }

  for _ in 0..m {
    if memo[0] == 0 {
      memo.pop_front();
    } else {
      break
    }
  }

  println!("{}", memo.into_iter().map(|v| v.to_string()).collect::<String>());
}