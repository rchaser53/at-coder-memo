/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

const DEFAULT:usize = 1_000_000_000_000;
fn helper(s:String,k:usize) -> usize {
  let arrs = s.split('x').map(|v| v.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
  let mut result = DEFAULT;

  for arr in arrs {
    if arr.len() < k { continue }
    let mut que = VecDeque::new();
    let mut d_num = 0;
    for v in arr {
      que.push_back(v);
      if v == '.' {
        d_num += 1;
      }

      if que.len() > k {
        if let Some(lv) = que.pop_front() {
          if lv == '.' {
            d_num -= 1;
          }
        }
      }
      if que.len() == k {
        result = result.min(d_num);
      }
    }
  }

  result
}

fn main() {
  input! {
    h:usize,
    w:usize,
    k:usize,
    rows:[String;h]
  }

  let mut result = DEFAULT;
  let mut new_rows = vec![];
  for row in rows {
    result = result.min(helper(row.clone(), k));
    new_rows.push(row.chars().collect::<Vec<char>>());
  }

  for j in 0..w {
    let mut cols = vec![];
    for i in 0..h {
      cols.push(new_rows[i][j]);
    }
    result = result.min(helper(cols.into_iter().map(|v| v.to_string()).collect::<String>(), k));
  }
  
  if result == DEFAULT {
    println!("-1");
  } else {
    println!("{}", result);
  }
}