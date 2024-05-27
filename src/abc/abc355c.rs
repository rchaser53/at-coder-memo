/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    t:usize,
    a:[Usize1;t]
  }

  let mut memo = vec![vec![false;n];n];
  let mut row = vec![0;n];
  let mut col = vec![0;n];
  let mut lr = 0;
  let mut rl = 0;

  let mut lr_set = HashSet::new();
  let mut rl_set = HashSet::new();
  for i in 0..n {
    lr_set.insert((i,i));
    rl_set.insert((i,n-1-i));
  }

  for i in 0..t {
    let v = a[i];
    let r = v / n;
    let c = v % n;

    if !memo[r][c] {
      memo[r][c] = true;

      row[r] += 1;
      col[c] += 1;
      if lr_set.contains(&(r,c)) {
        lr += 1;
      }
      if rl_set.contains(&(r,c)) {
        rl += 1;
      }
      
      if row[r] == n || col[c] == n || lr == n || rl == n {
        println!("{}", i+1);
        return
      }
    }
  }

  println!("-1");
}