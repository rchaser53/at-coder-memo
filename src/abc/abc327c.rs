/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    a:[[Usize1;9];9]
  }

  for i in 0..9 {
    let mut memo_c = vec![false;9];
    let mut memo_r = vec![false;9];
    for j in 0..9 {
      memo_c[a[i][j]] = true;
      memo_r[a[j][i]] = true;
    }

    for i in 0..9 {
      if !memo_c[i] || !memo_r[i] {
        println!("No");
        return
      }
    }
  }

  let p = [(0,0),(0,3),(0,6),(3,0),(3,3),(3,6),(6,0),(6,3),(6,6)];

  for (si,sj) in p {
    let mut memo = vec![false;9];
    for i in 0..3 {
      for j in 0..3 {
        memo[a[si+i][sj+j]] = true;
      }
    }

    for i in 0..9 {
      if !memo[i] {
        println!("No");
        return
      }
    }
  }
  println!("Yes");

}