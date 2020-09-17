#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
 
#[fastout]
fn main() {
  input! {
    s: String,
    mut k: usize
  }
  
  let s = s.chars().collect::<Vec<char>>();
  for i in 0..s.len() {
    let c = s[i];
    if c == '1' {
      k -= 1;
    } else {
      println!("{}", c);
      return
    }
    
    if k == 0 {
      println!("1");
      return
    }
  }
}