/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    mut ab:[[Usize1;2];n]
  }

  for i in 0..n {
    if ab[i][0] > ab[i][1] {
      ab[i].swap(0,1);
    }
  }
  ab.sort_by(|a,b| a[0].cmp(&b[0]));
  let mut memo = vec![0;2*n];

  for i in 0..n {
    let a = ab[i][0];
    let b = ab[i][1];
    memo[a] = i;
    memo[b] = i;
  }

  let mut que = VecDeque::new();
  let mut set = HashSet::new();
  for i in 0..2*n {
    let ti = memo[i];

    if que.is_empty() {
      que.push_back(ti);
      set.insert(ti);
    } else {
      let li = que[que.len()-1];
      if li == ti {
        que.pop_back();
        set.remove(&ti);
      } else if set.contains(&ti) {
        println!("Yes");
        return
      } else {
        que.push_back(ti);
        set.insert(ti);
      }
    }
  }

  println!("No");
}