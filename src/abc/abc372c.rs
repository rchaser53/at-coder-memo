/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    mut s:Chars,
    xc:[(Usize1,char);q]
  }

  let mut set = HashSet::new();
  for i in 0..n-2 {
    if set.contains(&i) { continue }
    if s[i] == 'A' && s[i+1] == 'B' && s[i+2] == 'C' {
      set.insert(i);
      set.insert(i+1);
      set.insert(i+2);
    }
  }

  for (x, c) in xc {
    if set.contains(&x) {
      if s[x] != c {
        if s[x] == 'A' {
          set.remove(&x);
          set.remove(&(x+1));
          set.remove(&(x+2));
        } else if s[x] == 'B' {
          set.remove(&(x-1));
          set.remove(&x);
          set.remove(&(x+1));
        } else if s[x] == 'C' {
          set.remove(&(x-2));
          set.remove(&(x-1));
          set.remove(&x);
        }
      }
    }
    
    s[x] = c;
    if !set.contains(&x) {
      if s[x] == 'A' && x < n-2 && s[x+1] == 'B' && s[x+2] == 'C'
      && !set.contains(&(x+1)) && !set.contains(&(x+2)) {
        set.insert(x);
        set.insert(x+1);
        set.insert(x+2);
      } else if s[x] == 'B' && 0 < x && x < n-1 && s[x-1] == 'A' && s[x+1] == 'C'
      && !set.contains(&(x-1)) && !set.contains(&(x+1)) {
        set.insert(x-1);
        set.insert(x);
        set.insert(x+1);
      } else if s[x] == 'C' && 1 < x && s[x-2] == 'A' && s[x-1] == 'B'
      && !set.contains(&(x-2)) && !set.contains(&(x-1)) {
        set.insert(x-2);
        set.insert(x-1);
        set.insert(x);
      }    
    }

    println!("{}", set.len() / 3);
  }

}