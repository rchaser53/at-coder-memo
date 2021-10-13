use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    n:usize,
    mut vals:[isize;3*n]
  }
  
  let mut memof = vec![0;n+1];

  let mut heap = BinaryHeap::new();
  let mut tot = 0isize;
  for i in 0..n {
    heap.push(Reverse(vals[i]));
    tot += vals[i];
  }
  memof[0] = tot;
  for i in 1..=n {
    let ci = n+i-1;
    let nv = vals[ci];
    let cv = heap.iter().next().unwrap().0;
    if cv < nv {
      tot += nv - cv;
      heap.pop();
      heap.push(Reverse(nv));
    }
    memof[i] = tot;
  }
  
  vals.reverse();
  let mut memob = vec![0;n+1];
  let mut heap = BinaryHeap::new();
  let mut tot = 0isize;
  for i in 0..n {
    heap.push(vals[i]);
    tot += vals[i];
  }
  memob[0] = tot;
  for i in 1..=n {
    let ci = n+i-1;
    let nv = vals[ci];
    let cv = *heap.iter().next().unwrap();
    if cv > nv {
      tot -= cv - nv;
      heap.pop();
      heap.push(nv);
    }
    memob[i] = tot;
  }
  
  let mut result = -1_000_000_000_000_000isize;
  for i in 0..=n {
    let v = memof[i] - memob[n-i];
    result = std::cmp::max(result, v);
  }
  println!("{}", result);
}