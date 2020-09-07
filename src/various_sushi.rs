use proconio::{input, fastout};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

#[fastout]
fn main() {
  input! {
    n: usize,
    k: usize,
    mut vals: [(usize, usize);n]
  }
  
  let mut map: HashMap<usize, Vec<usize>> = HashMap::new();  
  for (i, v) in vals {
    let entry = map.entry(i).or_insert(vec![]);
    entry.push(v);
  }
  let kinds = map.len();
  
  let mut stack: Vec<(usize, usize)> = vec![];

  for (_, smap) in map.iter_mut() {
    smap.sort();
    smap.reverse();
    stack.push((*smap.first().unwrap(), 1));
    for i in 1..smap.len() {
      stack.push((smap[i], 0));
    }
  }
  stack.sort_by(|a, b| a.0.cmp(&b.0));
  stack.reverse();

  let mut max = 0;
  let mut total = 0;
  let mut count = 0;
  let mut zeros = BinaryHeap::new();
  for i in 0..k {
    let (val, first) = stack[i];
    total += val;
    count += first;
    if first == 0 {
      zeros.push(Reverse(val));
    }
  }
  max = total + (count * count);
  let mut index = k;
  while count < kinds && count < k {
    let (val, first) = stack[index];
    if first == 1 {
      let minus = zeros.pop().unwrap();
      total = total - minus.0 + val;
      count += 1;
      max = std::cmp::max(max, total + (count * count));
    }
    index += 1;
  }
  
  println!("{}", max);
}