#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
  }
  
  let s = n.to_string().chars().collect::<Vec<char>>();
  let len = s.len();

  let base = vec![
    		0,
    		1,
    	   20,
  	      300,
         4000,
        50000,
       600000,
      7000000,
     80000000,
    900000000,
  ];
  
  let mut i = 0;
  while 10usize.pow(i) <= n {
    i += 1;
  }
  let i = i as usize;

  let mut result = 0;
  let mut one_count = 0;
  for j in 0..i {
    let v = s[j].to_string().parse::<usize>().unwrap();
    if 0 < v {
      let bv = 10usize.pow((len-j-1) as u32);
      if 1 < v {
        result += bv;
      }
      result += (base[len-j] - bv) / 10 * v;
      result += one_count * v * bv;
      
      if 1 == v {
        one_count += 1;
        result += 1;
      }
    }
  }  
  println!("{}", result);
}