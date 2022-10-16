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
    h:usize,
    w:usize,
    rs:usize,
    cs:usize,
    n:usize,
    rc:[(usize,usize);n],
    q:usize,
    dl:[(char, usize);q]
  }

  let mut rows = HashMap::new();
  let mut cols = HashMap::new();


  for (r,c) in rc {
    rows.entry(r).or_insert(vec![]).push(c);
    cols.entry(c).or_insert(vec![]).push(r);
  }

  for (_, arr) in rows.iter_mut() {
    arr.push(0);
    arr.sort();
    arr.push(w+1);
  }
  for (_, arr) in cols.iter_mut() {
    arr.push(0);
    arr.sort();
    arr.push(h+1);
  }

  let mut now = (rs, cs);
  for (d, l) in dl {
    if d == 'L' {
      let rv = if let Some(arr) = rows.get(&now.0) {
        let ti = lower_bound(&arr, &now.1);
        arr[ti-1]       
      } else {
        0
      };
      let av = now.1.saturating_sub(l);
      if rv < av {
        now.1 = av;
      } else {
        now.1 = rv+1;
      }
    }

    if d == 'R' {
      let rv = if let Some(arr) = rows.get(&now.0) {
        let ti = lower_bound(&arr, &now.1);
        arr[ti]
      } else {
        w+1
      };
      let av = now.1 + l;
      if av < rv {
        now.1 = av;
      } else {
        now.1 = rv - 1;
      }
    }

    if d == 'U' {
      let rv = if let Some(arr) = cols.get(&now.1) {
        let ti = lower_bound(&arr, &now.0);
        arr[ti-1]
      } else {
        0
      };
      let av = now.0.saturating_sub(l);
      if rv < av {
        now.0 = av;
      } else {
        now.0 = rv+1;
      }
    }

    if d == 'D' {
      let rv = if let Some(arr) = cols.get(&now.1) {
        let ti = lower_bound(&arr, &now.0);
        arr[ti]
      } else {
        h+1
      };

      let av = now.0 + l;
      if av < rv {
        now.0 = av;
      } else {
        now.0 = rv-1;
      }
    }

    println!("{} {}", now.0, now.1);
  }

}