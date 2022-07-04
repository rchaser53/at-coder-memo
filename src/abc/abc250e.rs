/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;


fn main() {
  input! {
    n:usize,
    a:[usize;n],
    b:[usize;n],
    q:usize,
    vals:[(Usize1,Usize1);q],
  }

  let mut map = HashMap::new();
  let mut memo_a = vec![0;n];

  for i in 0..n {
    let av = a[i];
    if map.get(&av).is_none() {
      let len = map.keys().len();
      map.insert(av, len+1);
    }
    memo_a[i] = map.keys().len();
  }

  let inf = 1_000_000_000_000usize;
  let mut memo_b = vec![(0,0);n];
  let mut max = 0;
  let mut set = HashSet::new();
  for i in 0..n {
    let bv = b[i];
    set.insert(bv);
    if let Some(&num) = map.get(&bv) {
      max = std::cmp::max(num, max);
    } else {
      max = inf;
    }
    memo_b[i] = (set.len(), max);
  }
  
  for i in 0..q {
    let (ai, bi) = vals[i];
    if memo_a[ai] == memo_b[bi].0 && memo_a[ai] == memo_b[bi].1 {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}