/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: UseValue, p: UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        let lv = arr[mid]+ x;
        match lv.cmp(&p) {
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
    m:usize,
    p:usize,
    a:[usize;n],
    mut b:[usize;m]
  }

  b.sort();
  let mut memo = vec![0;m+1];
  for i in 0..m {
    memo[i+1] = memo[i] + b[i];
  }

  let mut result = 0;
  for v in a {
    if p <= v {
      result += p * m;
    } else {
      let ti = lower_bound(&b, v, p);
      let av = if m <= ti {
        memo[m] + m * v
      } else if memo[ti] + v == p {
        memo[ti] + ti*v + (m-ti) * p
      } else {
        memo[ti] + ti*v + (m-ti) * p
      };
      result += av;      
    }
  }

  println!("{}", result);
}