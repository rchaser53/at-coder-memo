/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn helper(a:&BTreeSet<usize>) -> usize {
  let min = a.iter().next().unwrap();
  let max = a.iter().rev().next().unwrap();
  max - min
}

fn main() {
  input! {
    n:usize,
    k:usize,
    p:[Usize1;n]
  }
  
  let mut p = p.into_iter().enumerate().map(|(i,v)| (v,i)).collect::<Vec<(usize,usize)>>();
  p.sort();

  let mut btree = BTreeSet::new();
  for i in 0..k {
    btree.insert(p[i].1);
  }

  let mut result = helper(&btree);
  for i in k..n {
    btree.remove(&p[i-k].1);
    btree.insert(p[i].1);
    result = result.min(helper(&btree));
  }
  println!("{}", result);
}