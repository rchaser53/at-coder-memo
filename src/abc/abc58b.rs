
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    o:Chars,
    e:Chars
  }
  let mut result = vec![];
  
  for i in 0..o.len() {
    result.push(o[i].to_string());
    if let Some(v) = e.get(i) {
      result.push(v.to_string());
    }
  }
  println!("{}", result.into_iter().collect::<String>());
}