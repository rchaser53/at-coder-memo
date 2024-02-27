/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars,
    q:usize,
    cd:[(char,char);q]
  }

  let mut map = HashMap::new();
  for i in 0..n {
    map.entry(s[i]).or_insert(vec![]).push(i);
  }

  for (c, d) in cd {
    let mut a = map.remove(&c).unwrap_or(vec![]);
    let mut b = map.remove(&d).unwrap_or(vec![]);

    if b.len() < a.len() {
      std::mem::swap(&mut a, &mut b);
    }

    for v in a {
      b.push(v);
    }
    map.insert(d, b);
  }

  let mut result = vec!['a';n];
  for (c, v) in map {
    for i in v {
      result[i] = c;
    }
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}