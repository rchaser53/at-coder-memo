#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
 
fn helper(
  result: &mut Vec<String>,
  map: &Vec<char>,
  n: usize,
  temp: Vec<usize>
) {
  if temp.len() == n {
    result.push(
      temp
      .into_iter()
      .map(|v| map[v].to_string())
      .collect::<String>()
    );
    return
  }
  
  let mut max = 0;
  for i in 0..temp.len() {
    max = std::cmp::max(max, temp[i]);
  }
  
  for i in 0..=max+1 {
    let mut new_temp = temp.clone();
    new_temp.push(i);
    helper(result, map, n, new_temp); 
  }
}

#[fastout]
fn main() {
  input!{
    n: usize
  }
  
  let mut result: Vec<String> = vec![];
  let mut map = vec!['a','b','c','d','e','f','g','h','i','j'];
  helper(&mut result, &map, n, vec![0]);
  
  for v in result {
    println!("{}", v);
  }
}