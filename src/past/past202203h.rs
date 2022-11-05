/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn size(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = find(parents, i) as usize;
  -1 * parents[ii]
}

fn find(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = i as usize;
  if parents[ii] < 0 {
    i
  } else {
    parents[ii] = find(parents, parents[ii]);
    parents[ii]
  }
}

fn connect(
  parents: &mut Vec<isize>,
  a: isize,
  b: isize
) -> bool {
  let mut pa = find(parents, a);
  let mut pb = find(parents, b);
  
  if pa == pb { return false }
  
  if size(parents, pa) < size(parents, pb) {
    let temp = pa;
    pa = pb;
    pb = temp;
  }
  
  let paa = pa as usize;
  let pbb = pb as usize;
  parents[paa] += parents[pbb];
  parents[pbb] = pa;
  
  true
}

fn main() {
  input! {
    n:usize,
    q:usize
  }

  let mut parents = vec![-1;n];
  let mut memos = vec![BTreeSet::new();n];
  for i in 0..n {
    memos[i].insert(i as isize);
  }

  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        a:Isize1,
        b:Isize1
      }

      let ori_a = find(&mut parents, a);
      let ori_b = find(&mut parents, b);

      if ori_a == ori_b { continue }

      connect(&mut parents, a, b);

      let next_parent = find(&mut parents, a);
      if next_parent == ori_a {
        let mut temps = BTreeSet::new();
        std::mem::swap(&mut temps, &mut memos[ori_b as usize]);
        for v in temps {
          memos[next_parent as usize].insert(v);
        }
      } else {
        let mut temps = BTreeSet::new();
        std::mem::swap(&mut temps, &mut memos[ori_a as usize]);
        for v in temps {
          memos[next_parent as usize].insert(v);
        }
      }
      
      continue
    }

    input! {
      a: Isize1
    }
    let pi = find(&mut parents, a);
    let len = size(&mut parents, pi) as usize;
    
    let mut result = vec![0;len];
    let mut i = 0;
    let iter = &mut memos[pi as usize].iter();
    while let Some(&v) = iter.next() {
      result[i] = v + 1;
      i += 1;
    }
    println!("{}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  }
}