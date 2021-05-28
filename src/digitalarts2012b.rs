/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    mut s:Chars
  }
  let s_len = s.len();

  let s_string = s.iter().map(|c| c.to_string()).collect::<String>();
  if s_string == "a" || s_string == "zzzzzzzzzzzzzzzzzzzz" {
    println!("NO");
    return
  }

  let mut map = HashMap::new();
  for c in &s {
    *map.entry(c).or_insert(0) += 1;
  }
  let keys = map.keys().clone().into_iter().map(|v| **v).collect::<Vec<char>>();
  let keys_len = keys.len();

  if keys_len == 1 {
    if map.get(&'a').is_some() {
      s.remove(1);
      s.remove(0);
      s.push('b');
    } else if s_len < 20 {
      s[0] = (s[0] as u8 - 1) as char;
      s.push('a');
    } else if s_len == 20 {
      s[0] = (s[0] as u8 - 1) as char;
      s[1] = (s[1] as u8 + 1) as char;
    }
  } else {
    let first = s[0];
    let mut ti = 0;
    for i in 1..s_len {
      if s[i] != first {
        ti = i;
        break
      }
    }
    s.swap(0, ti);
  }
  println!("{}", s.iter().map(|c| c.to_string()).collect::<String>());
}
