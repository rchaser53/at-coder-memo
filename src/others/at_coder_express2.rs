#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
 
#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    q: usize,
    trains: [(usize,usize);m],
    queries: [(usize,usize);q]
  }
  
  let mut memo: Vec<Vec<usize>> = vec![vec![0;n+1];n+1];
  for (l, r) in trains {
    memo[l][r] += 1;
  }
  
  for i in 1..=n {
    for ii in 1..=n {
      memo[i][ii] += memo[i-1][ii];
      memo[i][ii] += memo[i][ii-1];
      memo[i][ii] -= memo[i-1][ii-1];
    }
  }
  
  for (l, r) in queries {
    let mut val = memo[r][r];
    val -= memo[l-1][r];
    val -= memo[r][l-1];
    val += memo[l-1][l-1];
    println!("{}", val);
  }
}

#[fastout]
fn my_answer() {
  input! {
    n: usize,
    m: usize,
    q: usize,
    trains: [(Usize1,Usize1);m],
    queries: [(Usize1,Usize1);q]
  }
  
  let mut memo = vec![vec![0;n];n];
  
  for (from, to) in trains {
    memo[from][to] += 1;
  }
  
  for i in (0..n).rev() {
    for ii in 0..n-1 {
      memo[i][ii+1] += memo[i][ii];
    }
  }
  
  for i in (1..n).rev() {
    for ii in 0..n {
      memo[i-1][ii] += memo[i][ii];
    }
  }
 
  for (from, to) in queries {
    println!("{}", memo[from][to]);
  }
}