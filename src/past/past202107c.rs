use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    mut s:Chars,
    l:usize,
    r:usize
  }

  if s.len() == 1 {
    let v = (s[0] as u8 - '0' as u8) as usize;
    if l <= v && v <= r {
      println!("Yes");
    } else {
      println!("No");
    }
    return
  } 

  if s[0] == '0' || 10 <= s.len() {
    println!("No");
    return
  }

  s.reverse();
  let mut v = 0;
  for i in 0..s.len() {
    v += (s[i] as u8 - '0' as u8) as usize * 10usize.pow(i as u32);
  }

  if l <= v && v <= r {
    println!("Yes");
  } else {
    println!("No");
  }
}