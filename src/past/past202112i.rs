/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    m:usize,
    edges:[(usize,usize,usize);m]
  }
  let mut map = HashMap::new();
  let mut set = HashSet::new();
  let inf = 10usize.pow(18) * 3;
  for (a,b,c) in edges {
    let entry = map.entry((a,b)).or_insert(inf);
    *entry = std::cmp::min(*entry, c);

    let entry = map.entry((b,a)).or_insert(inf);
    *entry = std::cmp::min(*entry, c);

    set.insert(a);
    set.insert(b);
  }
  set.insert(1);
  set.insert(n);

  let mut arr = set.into_iter().collect::<Vec<usize>>();
  arr.sort();
  let n = arr.len();
  let mut dict = HashMap::new();

  for i in 0..n {
    dict.insert(arr[i], i);
  }

  let mut g = vec![vec![];n];

  for i in 0..n-1 {
    let a = arr[i];
    let b = arr[i+1];
    let bv = arr[i+1] - arr[i];
    let v = if let Some(v) = map.get(&(a,b)) {
      std::cmp::min(*v, bv)
    } else {
      bv
    };
    g[i].push((i+1,v));
    g[i+1].push((i,v));
  }
  for ((a,b), c) in map {
    let ai = *dict.get(&a).unwrap();
    let bi = *dict.get(&b).unwrap();
    g[ai].push((bi, c));
    g[bi].push((ai, c));
  }

  let mut stack = vec![(0,0)];
  let mut memo = vec![inf;n];
  memo[0] = 0;
  while !stack.is_empty() {
    let mut new_stack = vec![];
    while let Some((ci,cv)) = stack.pop() {
      for &(ni, v) in &g[ci] {
        let nv = cv + v;
        if nv < memo[ni] {
          memo[ni] = nv;
          new_stack.push((ni,nv));
        }
      }
    }
    stack = new_stack;
  }

  println!("{}", memo[n-1]);
}