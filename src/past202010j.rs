#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    ab: usize,
    ac: usize,
    bc: usize,
    s: Chars,
    mut vals: [(Usize1, Usize1, usize);m]
  }
  
  let inf = 1_000_000_000_000_000_000;
  let ain = n;
  let bin = n+1;
  let cin = n+2;
  let aout = n+3;
  let bout = n+4;
  let cout = n+5;
  
  let mut ad_list = adjacency_list(n, &vals);
  for i in 0..6 {
    ad_list.push(vec![]);
  }
  for i in 0..n {
    match s[i] {
      'A' => {
        ad_list[i].push((ain, 0));
        ad_list[aout].push((i, 0));
      },
      'B' => {
        ad_list[i].push((bin, 0));
        ad_list[bout].push((i, 0));
      },
      _ => {
        ad_list[i].push((cin, 0));
        ad_list[cout].push((i, 0));
      }
    }
  }
  
  // Ain:0  Bin:1  Cin:2
  // Aout:3 Bout:4 Cout:5
  ad_list[ain].push((bout, ab)); // A => B
  ad_list[ain].push((cout, ac)); // A => C
  ad_list[bin].push((aout, ab)); // B => A
  ad_list[bin].push((cout, bc)); // B => C
  ad_list[cin].push((aout, ac)); // C => A
  ad_list[cin].push((bout, bc)); // C => B
    
  let result = dijkstra(n+6, inf, &ad_list, 0);  
  println!("{}", result[n-1]);
}

fn adjacency_list(
  n: usize,
  uvw: &Vec<(usize, usize, usize)>
) -> Vec<Vec<(usize, usize)>> {
  let mut e = vec![vec![]; n];
  for &(u, v, w) in uvw.iter() {
    e[u].push((v, w));
    e[v].push((u, w));
  }
  e
}

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize)>>,
  start: usize
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((w_u, u))) = pq.pop() {
    if score[u] < w_u {
      continue
    }
    
    for &(v, w_v) in graph[u].iter() {
      let w = w_u + w_v;
      if w < score[v] {
        score[v] = w;
        pq.push(std::cmp::Reverse((w, v)));
      }
    }
  }
  score
}


#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    ab: usize,
    ac: usize,
    bc: usize,
    s: Chars,
    mut vals: [(Usize1, Usize1, usize);m]
  }
  
  let ain = n;
  let bin = n+1;
  let cin = n+2;
  let aout = n+3;
  let bout = n+4;
  let cout = n+5;
  
  for i in 0..m {
    let (from, to, v) = vals[i];
    vals.push((to, from, v));
  }
  
  for i in 0..n {
    match s[i] {
      'A' => {
        vals.push((i, ain, 0));
        vals.push((aout, i, 0));
      },
      'B' => {
        vals.push((i, bin, 0));
        vals.push((bout, i, 0));
      },
      _ => {
        vals.push((i, cin, 0));
        vals.push((cout, i, 0));
      }
    }
  }
  
  // Ain:0  Bin:1  Cin:2
  // Aout:3 Bout:4 Cout:5
  vals.push((ain, bout, ab)); // A => B
  vals.push((ain, cout, ac)); // A => C
  vals.push((bin, aout, ab)); // B => A
  vals.push((bin, cout, bc)); // B => C
  vals.push((cin, aout, ac)); // C => A
  vals.push((cin, bout, bc)); // C => B
  
  let g = DiGraph::<usize, usize, usize>::from_edges(&vals);
  let res = dijkstra(&g, 0.into(), Some((n-1).into()), |e| *e.weight());
  let v = res.get(&NodeIndex::new(n-1)).unwrap();
  println!("{}", v);
}
