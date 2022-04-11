/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn helper(arr: Vec<usize>, max:usize, min:usize) -> usize {
  let n = arr.len();
  // let mut que = VecDeque::new();
  let mut max_num = 0;
  let mut min_num = 0;

  let mut li = 0;
  let mut ri = 0;
  let mut result = 0;

  while ri < n {
    let v = arr[ri];
    if v == min {
      min_num += 1;
    }
    if v == max {
      max_num += 1;
    }

    // minとmaxが1つ以上含まれている時
    while 0 < min_num && 0 < max_num && li <= ri {
      result += n - ri;

      let lv = arr[li];
      if lv == min {
        min_num -= 1;
      }
      if lv == max {
        max_num -= 1;
      }

      li += 1;
    }

    if ri < li {
      ri = li;
    } else {
      ri += 1;
    }
  }

  while 0 < min_num && 0 < max_num && li < n {
    result += 1;
    let lv = arr[li];
    if lv == min {
      min_num -= 1;
    }
    if lv == max {
      max_num -= 1;
    }

    li += 1;
  }

  result
}

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
    vals:[usize;n]
  }

  let mut result = 0usize;
  let mut memo = vec![vec![]];

  for v in vals {
    let li = memo.len() - 1;
    if v < y || x < v {
      if !memo[li].is_empty() {
        memo.push(vec![]);
      }
    } else {
      memo[li].push(v);
    }
  }

  for arr in memo {
    result += helper(arr, x, y);
  }

  println!("{}", result);
}