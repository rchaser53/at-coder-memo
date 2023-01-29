/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input!{
    s:Chars,
    t:Chars
  }

  let n = s.len();
  let m = t.len();
  let mut set = HashSet::new();
  for i in 0..m {
    let si = n-1-i;
    let ti = m-1-i;
    if s[si] != '?' && t[ti] != '?' && s[si] != t[ti] {
      set.insert(si);
    }
  }

  if set.len() == 0 {
    println!("Yes");
  } else {
    println!("No");
  }

  for i in 1..=m {
    let ai = i-1;

    if s[ai] != '?' && t[ai] != '?' && s[ai] != t[ai] {
      set.insert(ai);
    }

    let di = n-m+i-1;
    set.remove(&di);
    if set.len() == 0 {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}