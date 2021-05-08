/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn emit_result(n: usize, a: usize) {
    let mut result = vec![];
    for i in 0..n {
        if (a >> i) & 1 == 1 {
            result.push(i + 1);
        }
    }
    println!(
        "{} {}",
        result.len(),
        result
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}

pub fn main(
) {
    input! {
      n:usize,
      mut vals:[usize;n]
    }

    for i in 0..n {
        vals[i] = vals[i] % 200;
    }

    let a = std::cmp::min(n, 8);
    let limit = 1 << a;
    let mut map = HashMap::new();
    for i in 1..limit {
        let mut temp = 0;
        for j in 0..a {
            if (i >> j) & 1 == 1 {
                temp += vals[j];
                temp %= 200;
            }
        }
        if let Some(v) = map.get(&temp) {
            println!("Yes");
            emit_result(a, *v);
            emit_result(a, i);
            return;
        } else {
            map.insert(temp, i);
        }
    }
    println!("No");
}
