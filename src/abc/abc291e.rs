/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn topological_sort(
  g: &Vec<Vec<usize>>,
  counts: &mut Vec<usize>,
) -> Option<Vec<usize>> {
  let mut result = vec![0;g.len()];
  let mut zeros = vec![];

  for i in 0..g.len() {
    if counts[i] == 0 {
      zeros.push(i);
    }
  }

  let mut count = 1;
  while !zeros.is_empty() {
    let mut new_zeros = vec![];

    if zeros.len() != 1 {
      return None
    }
    
    while let Some(ci) = zeros.pop() {
      result[ci] = count;
      count += 1;
      for &ni in &g[ci] {
        counts[ni] -= 1;
        if counts[ni] == 0 {
          new_zeros.push(ni);
        }
      }
    }
    zeros = new_zeros;
  }
  Some(result)
}

fn main() {
  input!{
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1);m]
  }

  let mut g = vec![vec![];n];
  let mut counts = vec![0;g.len()];
  for (a,b) in edges {
    g[a].push(b);
    counts[b] += 1;
  }

  if let Some(arr) = topological_sort(&g, &mut counts) {
    println!("Yes");
    println!("{}", arr.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "))
  } else {
    println!("No");
  }
}