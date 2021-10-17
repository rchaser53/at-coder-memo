use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn topological_sort(
  g: &Vec<Vec<usize>>,
  counts: &mut Vec<usize>,
) -> Vec<usize> {
  let mut result = vec![];
  // BinaryHeapを使うと辞書順にできる
  let mut zeros = BinaryHeap::new();

  for i in 0..g.len() {
    if counts[i] == 0 {
      zeros.push(Reverse(i));
    }
  }

  while let Some(Reverse(ci)) = zeros.pop() {
    result.push(ci);
    for &ni in &g[ci] {
      counts[ni] -= 1;
      if counts[ni] == 0 {
        zeros.push(Reverse(ni));
      }
    }
  }
  result
}

fn main() {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  
  // ここでnodeから出ているedgeの数のcountsを作るのを忘れないこと
  let mut set = HashSet::new();
  let mut g = vec![vec![];n];
  let mut counts = vec![0;n];
  for i in 0..m {
    let (a, b) = vals[i];
    if !set.contains(&(a,b)) {
      g[a].push(b);
      counts[b] += 1;
      set.insert((a,b));
    }
  }

  let result = topological_sort(&g, &mut counts);
  if result.len() != n {
    println!("-1");
  } else {
    let result = result.into_iter().map(|v| (v+1).to_string()).collect::<Vec<String>>();
    println!("{}", result.join(" "));
  }
}