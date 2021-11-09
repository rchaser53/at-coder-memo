/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n:usize,
    k:usize,
    l:usize,
    edges1:[(Usize1,Usize1);k],
    edges2:[(Usize1,Usize1);l]
  }

  let mut tree: UnionFind<usize> = UnionFind::new(n);
  let mut map_a = HashMap::new();
  for &(a, b) in &edges1 {
    tree.union(a,b);
    tree.union(b,a);
  }
  for i in 0..n {
    map_a.entry(tree.find(i)).or_insert(vec![]).push(i);
  }

  let mut tree2: UnionFind<usize> = UnionFind::new(n);
  for &(a, b) in &edges2 {
    tree2.union(a,b);
    tree2.union(b,a);
  }

  let mut seen = vec![false;n];
  let mut result = vec![1;n];
  for i in 0..n {
    if seen[i] { continue }
    
    let mut map = HashMap::new();
    for &j in map_a.get(&tree.find(i)).unwrap() {
      seen[j] = true;
      map.entry(tree2.find(j)).or_insert(vec![]).push(j);
    }

    for (_, arr) in map {
      let len = arr.len();
      for j in arr {
        result[j] = len;
      }
    }
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}