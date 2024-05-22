/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    a:[(usize,usize);n],
  }

  let mut a = a.into_iter().enumerate().collect::<Vec<(usize, (usize, usize))>>();
  a.sort_by(|a, b| (a.1).0.cmp(&(b.1).0));

  let mut bset = BTreeSet::new();
  for i in 0..n-1 {
    // cost, index
    bset.insert((a[i].1.1, a[i].0));
  }

  let mut keep = vec![];
  let mut trashed = HashSet::new();
  for _ in 0..n {
    let (i1, (_,c1)) = a.pop().unwrap();
    if trashed.contains(&i1) { continue }

    while !bset.is_empty() {
      let &(c2, i2) = bset.iter().rev().next().unwrap();
      if c1 < c2 {
        bset.remove(&(c2, i2));
        trashed.insert(i2);
      } else {
        break
      }
    }

    bset.remove(&(c1, i1));
    keep.push(i1+1);
  }

  keep.sort();
  let result = keep.into_iter().map(|v| v.to_string()).collect::<Vec<String>>();
  println!("{}", result.len());
  println!("{}", result.join(" "));
}
