use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
  input! {
    N: usize,
    M: usize,
    AB: [(Usize1, Usize1);M]
  }
  let mut neighbors: Vec<Vec<usize>> = vec![Vec::with_capacity(N); N];
  let mut result: Vec<usize> = vec![N; N];
  
  for (a, b) in AB {
    neighbors[a].push(b);
    neighbors[b].push(a);
  }

  let mut nexts: VecDeque<usize> = VecDeque::new();
  nexts.push_back(0);
  while let Some(value) = nexts.pop_front() {
    for &nei_val in neighbors[value].iter() {
      if result[nei_val] == N {
          nexts.push_back(nei_val);
            result[nei_val] = value;
        }
    }
  }

  println!("Yes");
  for v in result.iter().skip(1) {
    println!("{}", v + 1);
  }
}