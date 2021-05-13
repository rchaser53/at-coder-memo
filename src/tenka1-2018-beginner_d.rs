/* OUTPUT FILE */
#![allow(unused_imports)]
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
    }

    let mut i = 1usize;
    let n2 = n * 2;
    while i * (i + 1) < n2 && i * (i + 1) != n2 {
        i += 1;
    }

    if i * (i + 1) != n2 {
        println!("No");
        return;
    }

    let mut result = vec![vec![0; i]; i + 1];

    for j in 0..i {
        let shim = (j + 1) * j / 2 + 1;
        for k in 0..=j {
            result[k][j] = k + shim;
        }
        for k in 0..=j {
            result[j + 1][k] = k + shim;
        }
    }
    println!("Yes");
    println!("{}", result.len());

    for i in 0..i + 1 {
        println!(
            "{} {}",
            result[i].len(),
            result[i]
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
