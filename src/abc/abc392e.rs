/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

struct UnionFind {
  parents: Vec<usize>,
  sizes: Vec<usize>,
  ranks: Vec<usize>
}

impl UnionFind {
  fn new(n:usize) -> UnionFind {
    let mut uf = UnionFind { parents: vec![0;n], sizes:vec![1;n], ranks:vec![0;n] };
    for i in 0..n {
      uf.parents[i] = i;
    }
    uf
  }

  fn root(&mut self, x:usize) -> usize {
    let ti = self.parents[x];
    if ti == x {
      x
    } else {
      self.parents[x] = self.root(ti);
      self.parents[x]
    }
  }

  fn is_same(&mut self, x:usize, y:usize) -> bool {
    self.root(x) == self.root(y)
  }

  fn unite(&mut self, x:usize, y:usize) -> bool {
    let rx = self.root(x);
    let ry = self.root(y);
    if rx == ry { return false }
    let (rx, ry) = if self.ranks[rx] < self.ranks[ry] {
      (ry, rx)
    } else {
      (rx, ry)
    };
    self.parents[ry] = rx;
    if self.ranks[rx] == self.ranks[ry] {
      self.ranks[rx] += 1;
    }

    self.sizes[rx] += self.sizes[ry];
    true
  }

  fn group_size(&mut self, x:usize) -> usize {
    let i = self.root(x);
    self.sizes[i]
  }
}

fn kruskal(
  edges: &Vec<(usize,usize,usize)>,
  n:usize,
) -> (UnionFind, Vec<(usize,usize,usize)>) {
  let mut tree = UnionFind::new(n);
  let mut result = vec![];
  for &(a,b,i) in edges {
    if !tree.unite(a, b) {
      result.push((a,b,i)); 
    }
  }
  (tree, result)
}

fn main() {
  input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
  }

  let mut tree = UnionFind::new(n);
  let mut new_ab = vec![];
  for i in 0..m {
    let (a,b) = ab[i];
    tree.unite(a,b);
    new_ab.push((a,b,i));
  }

  // let mut group = HashMap::new();
  // for i in 0..m {
  //   let (a,b) = ab[i];
  //   let pi = tree.root(a);
  //   group.entry(pi).or_insert(vec![]).push((a,b,i));
  // }

  let (mut tree, available_edges) = kruskal(&new_ab, n);
  let mut group = HashMap::new();
  for edge in available_edges {
    let pi = tree.root(edge.0);
    group.entry(pi).or_insert(vec![]).push(edge);
  }

  if group.is_empty() {
    println!("0");
    return
  }

  let mut bset = BTreeSet::new();
  let mut seen = vec![false; n];
  let mut only_one_group = vec![];
  for i in 0..n {
    let pi = tree.root(i);
    if seen[pi] { continue }
    seen[pi] = true;

    if let Some(edges) = group.get(&pi) {
      bset.insert((edges.len(), edges.clone(), pi));
    } else {
      only_one_group.push(pi);
    }
  }

  let mut result = vec![];
  while bset.len() > 1 {
    let (_, min_edges, min_i) = bset.pop_first().unwrap();
    let (_, mut max_edges, max_i) = bset.pop_last().unwrap();

    let is_min_large_group = tree.group_size(min_i) > tree.group_size(max_i);
    if tree.unite(min_i, max_i) {
      let (a,_,i) = max_edges.pop().unwrap();
      result.push((i+1, a+1, min_i+1));
    }

    for (a,b,i) in min_edges {
      max_edges.push((a,b,i));
    }

    if is_min_large_group {
      bset.insert((max_edges.len(), max_edges, min_i));
    } else {
      bset.insert((max_edges.len(), max_edges, max_i));
    }
  }

  let mut max_edges = bset.pop_first().unwrap_or((0,vec![],1)).1;
  for ti in only_one_group {
    let (a,_,i) = max_edges.pop().unwrap();
    result.push((i+1, a+1, ti+1));
  }
  
  println!("{}", result.len());
  for (i,a,b) in result {
    println!("{} {} {}", i, a, b);
  }
}