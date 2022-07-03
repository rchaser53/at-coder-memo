/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &[Target], x: &UseValue, padding:UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        match (arr[mid] - padding).cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

fn helper(memo: &Vec<usize>, ci:usize) -> usize {
  if ci == 0 {
    0
  } else {
    memo[ci-1]
  }
}

fn main() {
  input! {
    n:usize,
    q:usize,
    x:usize,
    vals:[usize;n],
    k:[usize;q]
  }

  let mut memo = vec![0;n];
  memo[0] = vals[0];
  for i in 1..n {
    memo[i] = memo[i-1] + vals[i];
  }
  
  let mut stack = vec![];
  let mut set = HashSet::new();
  let mut ci = 0;
  let tot = memo[n-1];

  let mut scores = vec![0;n];
  while !set.contains(&ci) {
    set.insert(ci);
    stack.push(ci);
    let padding = helper(&memo, ci);
    let left = tot - padding;
    let temp = if left <= x {
      let remove_left_x = x - left;
      let looped_count = remove_left_x / tot;
      let left_need = remove_left_x % tot;

      if left_need == 0 {
        scores[ci] = n - ci + looped_count * n;
        n - 1
      } else {
        let i = lower_bound(&memo, &left_need, 0);

        scores[ci] = n - ci + looped_count * n + i + 1;
        i
      }
    } else {
      let i = lower_bound(&memo[ci..], &x, padding) + ci;
      scores[ci] = i - ci + 1;
      i
    } + 1;
    ci = temp % n;
  }
  
  let mut tail = vec![];
  let mut loop_arr = vec![];
  let mut i = 0;
  while i < stack.len() {
    let ti = stack[i];
    if ci == ti {
      while i < stack.len() {
        loop_arr.push(stack[i]);
        i += 1;
      }
      break
    }
    tail.push(ti);
    i += 1;
  }

  for v in k {
    if v <= tail.len() {
      println!("{}", scores[tail[v-1]]);
    } else {
      let v = v - tail.len() - 1;
      println!("{}", scores[loop_arr[v % loop_arr.len()]]);
    }
  }
}