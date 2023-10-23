/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut t:Chars,
  }

  let mut t = t.into_iter().map(|v| v as usize - '0' as usize).collect::<Vec<usize>>();
  t.sort();

  let mut max = 0usize;
  for i in 0..n {
    max += t[i] * 10usize.pow(i as u32);
  }

  let mut memo = vec![0;10];
  for &v in &t {
    memo[v] += 1;
  }

  let mut dict = vec![];
  for i in 0..13 {
    let mut a = format!("");
    for _ in 0..i {
      a = format!("0{}", a);
    }
    dict.push(a);
  }

  let mut i = 0;
  let mut result = 0;
  while i * i <= max {
    let v = i * i;
    let mut temp = vec![0;10];
    let s = v.to_string();
    let m = n - s.len();
    let s = format!("{}{}", &dict[m], s);
    
    for c in s.chars() {
      temp[c as usize - '0' as usize] += 1;
    }

    let mut success = true;
    for j in 0..10 {
      if temp[j] != memo[j] {
        success = false;
        break
      }
    }
    if success {
      result += 1;
    }
    i += 1;
  }
  println!("{}", result);
}