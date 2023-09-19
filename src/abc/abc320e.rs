/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    tws:[(usize,usize,usize);m]
  }

  let mut result = vec![0;n];
  let mut bset1 = BTreeSet::new();
  let mut bmap: BTreeMap<usize, (Vec<(usize,usize)>, Vec<(usize,usize)>)> = BTreeMap::new();
  for i in 0..n {
    bset1.insert(i);
  }

  for &(t,w,s) in &tws {
    bmap.insert(t,(vec![(w,s)],vec![]));
  }

  while let Some((time,(arr1,arr2))) = bmap.pop_first() {
    for v in arr2 {
      bset1.insert(v.0);
    }

    for (aw,asec) in arr1 {
      if let Some(ti) = bset1.pop_first() {
        result[ti] += aw;
        
        let entry = bmap.entry(time + asec).or_insert((vec![],vec![]));
        entry.1.push((ti,0));
      }
    }
  }

  for v in result {
    println!("{}", v);
  }

}