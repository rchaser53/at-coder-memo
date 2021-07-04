use petgraph::unionfind::UnionFind;
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
      vals:[usize;n]
    }

    let mut map = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    let mut max = 0;
    while right < n {
        while right < n && map.keys().len() <= m {
            let v = vals[right];
            *map.entry(v).or_insert(0) += 1;
            right += 1;
        }

        if m < map.keys().len() {
            max = std::cmp::max(max, right - left - 1);
        } else {
            max = std::cmp::max(max, right - left);
        }

        while left < right && m < map.keys().len() {
            let v = vals[left];
            *map.entry(v).or_insert(0) -= 1;
            if map.get(&v).unwrap() == &0 {
                map.remove(&v);
            }
            left += 1;
        }
    }
    println!("{}", max);
}
