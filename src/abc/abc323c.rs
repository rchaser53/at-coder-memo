/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;m],
    ss: [Chars;n]
  }

  let mut max_count = 0;
  let mut max = 0;
  let mut memo = vec![(0usize,vec![]);n];
  for i in 0..n {
    let mut score = i+1;
    let mut temp = vec![];
    for j in 0..m {
      if ss[i][j] == 'x' {
        temp.push(a[j]);
      } else {
        score += a[j];
      }
    }
    temp.sort();
    temp.reverse();
    memo[i] = (score, temp);

    if max == score {
      max_count += 1;
    } else if max < score {
      max = score;
      max_count = 1;
    }
  }

  for i in 0..n {
    let mut now = memo[i].0;
    let arr = &memo[i].1;
    let len = arr.len();
    let mut j = 0;

    while j < len {
      if 1 < max_count {
        if now > max {
          break
        }
      } else {
        if now >= max {
          break
        }
      }

      now += arr[j];
      j += 1;
    }
    println!("{}", j);
  }  
}