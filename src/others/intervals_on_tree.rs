use proconio::input;
use std::collections::HashMap;
 
fn main() {
  input! {
    n: usize,
    a: [(usize, usize);n-1],
  }
  
  let mut vertex = 0;
  for i in 1..=n {
    vertex += i * (n - i + 1);
  }
  
  let mut edges = 0;
  for (u, v) in a {
    edges += if u > v {
       v * (n - u + 1) 
    } else {
       u * (n - v + 1)
    };
  }
  
  println!("{}", vertex - edges);  
}
