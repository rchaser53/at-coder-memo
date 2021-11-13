#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: String
  }
  
  if String::from("WBWBWWBWBWBWWBWBWWBW") == s {
    println!("Do");
    return
  }
  
  if String::from("WBWWBWBWBWWBWBWWBWBW") == s {
    println!("Re");
    return
  }
  
  if String::from("WWBWBWBWWBWBWWBWBWBW") == s {
    println!("Mi");
    return
  }
  
  if String::from("WBWBWBWWBWBWWBWBWBWW") == s {
    println!("Fa");
    return
  }
  
  if String::from("WBWBWWBWBWWBWBWBWWBW") == s {
    println!("So");
    return
  }
  
  if String::from("WBWWBWBWWBWBWBWWBWBW") == s {
    println!("La");
    return
  }
  
  if String::from("WWBWBWWBWBWBWWBWBWWB") == s {
    println!("Si");
    return
  }
}