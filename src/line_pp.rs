use proconio::input;
use std::collections::VecDeque;
use proconio::marker::Usize1;

fn push(dist: &mut Vec<usize>, queue: &mut VecDeque<usize>, v: usize, d: usize) {
  if dist[v] != usize::max_value() {
    return
  }
  dist[v] = d;
  queue.push_back(v);
}

fn main() {
  input! {
    K: usize,
    X: Usize1,
    Y: Usize1,
  }
  
  let mut answer: Vec<usize> = vec![0;K];
  for i in 0..K {
    let mut dist = vec![usize::max_value();K];
    let mut queue = VecDeque::new();
    push(&mut dist, &mut queue, i, 0);
    while queue.len() > 0 {
      let v = queue.pop_front().unwrap();
      let d = dist[v];
      if 0 != v { push(&mut dist, &mut queue, v-1, d+1); }
      if v+1 < K { push(&mut dist, &mut queue, v+1, d+1); }
      if v == X { push(&mut dist, &mut queue, Y, d+1); }
      if v == Y { push(&mut dist, &mut queue, X, d+1); }
    }
    
    for ii in dist {
      answer[ii] += 1;
    }
  }

  for i in 1..K {
    println!("{}", answer[i]/2);
  }
}
