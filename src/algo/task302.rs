#![allow(unused_imports, dead_code)]
use std::{collections::*, ops::RangeBounds};
fn main() {
  let a = readvec::<usize>();
	let n = a[0];
	let mut l1 = a[1];
	let mut l2 = a[2];

	for _ in 0..n-1 {
		let temp = l2;
		l2 = (l1 + l2) % 100;
		l1 = temp;
	}

    println!("{}", l1);
}

fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
}
fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
}
fn join_space<T: ToString>(v: Vec<T>) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
