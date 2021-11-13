#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
 
#[fastout]
fn main() {
  input!{
    s: Chars
  }
  
  let mut memo = vec![0;s.len()+1];
  for i in 0..s.len() {
    if s[i] == '<' {
      memo[i+1] = memo[i]+1;
    }
  }
  
  for i in (0..s.len()).rev() {
    if s[i] == '>' {
      memo[i] = std::cmp::max(memo[i], memo[i+1]+1);
    }
  }
    
  println!("{}", memo.iter().sum::<usize>()); 
}