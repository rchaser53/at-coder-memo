/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut result = 1;
  for i in 1..n-1 {
    if s[i] == '/' {
      let ii = i as isize;
      let mut x = 0;
      while 0 <= ii-x-1 && ii+x+1 < n as isize {
        let ui = ii as usize;
        let ux = x as usize;
        let li = ui-ux-1;
        let ri = ui+ux+1;
        if s[li] == '1' && s[ri] == '2' {
          result = result.max(1+(x+1)*2);
        } else {
          break
        }
        x += 1;
      }
    }
  }
  
  println!("{}", result);
}