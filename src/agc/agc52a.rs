use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    t:usize
  }

  for _ in 0..t {
    input!{
      n:usize,
      _:Chars,
      _:Chars,
      _:Chars,
    }

    let a = vec![String::from("1");n].into_iter().collect::<String>();
    let b = vec![String::from("0");n].into_iter().collect::<String>();
    
    println!("{}{}1", a,b);
  }
}