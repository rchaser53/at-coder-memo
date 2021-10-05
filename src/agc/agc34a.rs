use proconio::input;
use proconio::marker::*;
use std::collections::*;
 
pub fn main(
) {
  input! {
    n:usize,
    a:Usize1,
    b:Usize1,
    c:Usize1,
    d:Usize1,
    s:Chars
  }
 
  let mut count = 0;
  for i in a..=c {
    if s[i] == '#' {
      count += 1;
    } else {
      count = 0;
    }
    if 1 < count {
      println!("No");
      return
    }
  }
 
  count = 0;
  let mut space = 0;
 
  if s[b-1] == '.' {
    space += 1;
  }
 
  let mut succeed = false;
  for i in b..=d {
    if s[i] == '#' {
      count += 1;
      space = 0;
    } else {
      space += 1;
      count = 0;
    }
 
    if 1 < count {
      println!("No");
      return
    }
 
    if 2 < space {
      succeed = true;
    }
  }
  if d < n - 1 && s[d+1] == '.' {
    if 2 < space + 1 {
      succeed = true;
    }
  }
 
  if d < c {
    if succeed {
      println!("Yes");
    } else {
      println!("No");
    }
    return
  }
  println!("Yes");
}