#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [[Usize1;n-1];n]
  }
  
  let mut count = 0;
  let mut memo = vec![(0, false);n];
  loop {
    let mut flag = true;
    for i in 0..n {
      memo[i].1 = false;
    }

    for i in 0..n {
      let from = memo[i];
      if from.0 == n-1 || from.1 { continue }

      let ti = vals[i][from.0];
      let to = memo[ti];
      if to.0 == n-1 || to.1 { continue }
      
      if vals[ti][to.0] != i { continue }
      memo[i] = (memo[i].0+1, true);
      memo[ti] = (memo[ti].0+1, true);
      flag = false;
    }
    count += 1;
    if flag { break }
  }
  for (v, _) in memo {
    if v != n-1 {
      println!("-1");
      return
    }
  }
  println!("{}", count-1);
}

#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::mem;

#[fastout]
fn main() {
  input!{
    n: usize,
    mut vals: [[Usize1;n-1];n]
  }
  
  for i in 0..n {
    vals[i].reverse();
  }
  let mut q = vec![];
  
  for i in 0..n {
    check(i, &mut q, &vals);
  }
  
  let mut count = 0;
  while !q.is_empty() {
    count += 1;
    q.sort();
    q.dedup();
    let mut prev_q = vec![];
    mem::swap(&mut q, &mut prev_q);
    for &p in prev_q.iter() {
      let i = p.0;
      let j = p.1;
      vals[i].pop();
      vals[j].pop();
    }
    
    for &p in prev_q.iter() {
      let i = p.0;
      let j = p.1;
      check(i, &mut q, &vals);
      check(j, &mut q, &vals);
    }
  }
  
  for i in 0..n {
    if !vals[i].is_empty() {
      println!("-1");
      return
    }
  }
  println!("{}", count);
}

fn check(
  i:usize,
  q:&mut Vec<(usize, usize)>,
  vals:&Vec<Vec<usize>>
) {
  if vals[i].is_empty() { return }
  
  let j = *vals[i].last().unwrap();
  if vals[j].is_empty() { return }
  
  if *vals[j].last().unwrap() == i {
    let mut p = (i, j);
    if p.1 < p.0 {
      mem::swap(&mut p.0, &mut p.1);
    }
    q.push(p);
  }
}