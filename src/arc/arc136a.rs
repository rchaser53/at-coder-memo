/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn helper(mut a:Vec<char>) -> Vec<char> {
  let mut result = vec![];
  a.sort();
  for c in a {
    if result.is_empty() {
      result.push(c);
    } else {
      let li = result.len() - 1;
      if result[li] == 'B' && c == 'B' {
        result[li] = 'A';
      } else {
        result.push(c);
      }
    }
  }
  result
}

fn main() {
  input! {
    n:usize,
    s:Chars,
  }

  let mut result = vec![];
  let mut temp = vec![];
  for c in s {
    if c == 'C' {
      let arr = helper(temp);
      for cc in arr {
        result.push(cc);
      }
      result.push(c);
      temp = vec![];
    } else {
      if temp.is_empty() {
        temp.push(c);
      } else {
        let li = temp.len() - 1;
        if temp[li] == 'B' && c == 'B' {
          temp[li] = 'A';
        } else {
          temp.push(c);
        }
      }
    }
  }

  let arr = helper(temp);
  for cc in arr {
    result.push(cc);
  }
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}