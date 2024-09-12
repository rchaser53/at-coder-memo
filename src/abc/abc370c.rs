/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    mut s:Chars,
    t:Chars
  }

  let n = s.len();
  let mut result = vec![];
  loop {
    let mut temp = vec![];
    for i in 0..n {
      if s[i] != t[i] {
        let mut new_s = s.clone();
        new_s[i] = t[i];
        temp.push(new_s);
      }
    }

    temp.sort();
    temp.reverse();
    if temp.is_empty() {
      break
    }
    let l = temp.pop().unwrap();
    result.push(l.clone());
    s = l;
  }

  println!("{}", result.len());
  for v in result {
    println!("{}", v.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}