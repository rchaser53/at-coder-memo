#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn main() {
  input!{
    mut s: Chars,
    t: Chars
  }
  
  if s.len() < t.len() {
    println!("UNRESTORABLE");
    return
  }
  
  for i in (0..=(s.len()-t.len())).rev() {
    let mut flag = true;
    for ii in 0..t.len() {
      if s[i+ii] == '?' || s[i+ii] == t[ii] {
        continue
      } else {
        flag = false;
        break
      }
    }
    
    if flag {
      for ii in 0..t.len() {
        s[i+ii] = t[ii];
      }
      for ii in 0..s.len() {
        if s[ii] == '?' {
          s[ii] = 'a';
        }
      }
      println!("{}", s
         .into_iter()
         .map(|v| v.to_string())
         .collect::<String>()
      );
      return
    }
  }
  println!("UNRESTORABLE");
}
