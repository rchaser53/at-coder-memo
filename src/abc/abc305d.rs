/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
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
    a:[usize;n],
    q:usize,
    lr:[(usize,usize);q]
  }

  let mut memo = vec![0;n];
  for i in 1..n {
    memo[i] = memo[i-1];
    if i % 2 == 0 {
      memo[i] += a[i] - a[i-1];
    }
  }

  for (l, mut r) in lr {
    let li = lower_bound(&a, &l);
    if n <= li {
      println!("0");
      continue
    }

    let mut ri = lower_bound(&a, &r);
    if ri == n {
      ri -= 1;
      r = a[n-1];
    }

    if li == ri {
      if li == 0 || memo[li-1] == memo[li] {
        println!("0");
      } else {
        println!("{}", r - l);
      }
    } else {
      let lv = if li == 0 || memo[li-1] == memo[li] {
        0
      } else {
        a[li] - l
      };

      let rv = if memo[ri-1] == memo[ri] {
        0
      } else {
        r - a[ri-1]
      };

      let cv = memo[ri-1] - memo[li];
      println!("{}", lv + cv + rv);
    }
  }
}