/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

type Target = isize;
type UseValue = isize;
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
  input!{
    n:usize,
    m:usize,
    d:isize,
    a:[isize;n],
    b:[isize;m]
  }
  let mut set = HashSet::new();
  for v in b {
    set.insert(v);
  }

  let mut b = set.into_iter().collect::<Vec<isize>>();
  let m = b.len();
  b.sort();

  let mut result = -1;
  for v1 in a {
    let mv = v1-d;
    let pv = v1+d;

    let mut temps = vec![];
    if 0 <= mv {
      temps.push(lower_bound(&b, &mv));
    }

    if 0 <= pv {
      temps.push(lower_bound(&b, &pv));
    }
    
    for i in temps {  
      for j in i.saturating_sub(3)..=(i+3).min(m-1) {
        let v2 = b[j];
        if (v1-v2).abs() <= d {
          result = result.max(v1+v2);
        }
      }
    }
  }

  println!("{}", result);
}