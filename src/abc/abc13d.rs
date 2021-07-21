/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    d:usize,
    vals:[usize;m]
  }

  let mut memoi = (0..n).into_iter().collect::<Vec<_>>();
  let mut memo = (0..n).into_iter().collect::<Vec<_>>();
  for i in 0..m {
    let v = vals[i];
    memoi.swap(v, v-1);
    let ti1 = memoi[v];
    let ti2 = memoi[v-1];
    memo.swap(ti1, ti2);
  }

  let mut map = vec![(0,0);n];
  let mut s_stack = vec![];
  let mut set = HashSet::new();
  for fi in 0..n {
    let mut ni = fi;
    let mut lgroup = vec![];
    while !set.contains(&ni) {
      set.insert(ni);
      lgroup.push(ni);
      ni = memo[ni];
    }

    if lgroup.is_empty() { continue }

    s_stack.push(lgroup);
    let gi = s_stack.len() - 1;
    let lgroup = &s_stack[gi];

    for i in 0..lgroup.len() {
      let ci = lgroup[i];
      map[ci] = (gi, i);
    }
  }

  for (gi, i) in map {
    let loop_group = &s_stack[gi];
    let ln = loop_group.len();
    println!("{}", loop_group[(i + d) % ln] + 1);
  }
}
