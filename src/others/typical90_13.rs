use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      m:usize,
      vals:[(Usize1,Usize1,usize);m]
    }

    let g = UnGraph::<usize, usize, usize>::from_edges(&vals);
    let first = dijkstra(&g, 0.into(), Some(n.into()), |e| *e.weight());
    let last = dijkstra(&g, (n-1).into(), Some(n.into()), |e| *e.weight());

    println!("{}", *first.get(&NodeIndex::new(n - 1)).unwrap());
    for i in 1..n-1 {
        let a = *first.get(&NodeIndex::new(i)).unwrap();
        let b = *last.get(&NodeIndex::new(i)).unwrap();
        println!("{}", a + b);
    }
    println!("{}", *first.get(&NodeIndex::new(n - 1)).unwrap());
}
