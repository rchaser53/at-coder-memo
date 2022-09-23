/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
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

fn main() {
  input! {
    n:usize,
    mut c:[usize;n],
    q:usize,
    queries:[usize;q]
  }

  c.sort();
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = memo[i] + c[i];
  }
  
  for v in queries {
    let ti = lower_bound(&memo, &v);
    if ti == n+1 {
      println!("{}", n);
    } else {
      if memo[ti] == v {
        println!("{}", ti);
      } else if memo[1] > v {
        println!("0");
      } else {
        println!("{}", ti-1);
      }
    }
  }
}