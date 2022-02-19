/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn main() {
  input! {
    n:Chars,
  }

  let mut result = 500usize;
  let mut last = n[0];
  let seta = ['1','2','3','4','5'].iter().collect::<HashSet<&char>>();
  let setb = ['0','6','7','8','9'].iter().collect::<HashSet<&char>>();
  for i in 1..n.len() {
    let c = n[i];
    if c == last {
      result += 301;
    } else {
      if seta.contains(&c) && seta.contains(&last) {
        result += 210;
      } else if setb.contains(&c) && setb.contains(&last) {
        result += 210;
      } else {
        result += 100;
      }
    }
    last = c;
  }
  println!("{}", result);
}