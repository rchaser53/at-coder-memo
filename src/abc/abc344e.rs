/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n],
    q:usize,
  }

  let mut map = HashMap::new();

  let none = 10usize.pow(15);
  let mut last = none;
  for i in 0..n-1 {
    let v = a[i];
    map.insert(v, (last, a[i+1]));
    last = v;
  }
  if 1 < n {
    map.insert(a[n-1], (a[n-2], none));
  }
  if 1 == n {
    map.insert(a[0], (none,none));
  }

  for _ in 0..q {
    input!{
      t: usize,
      x: usize
    }

    if t == 1 {
      input! {
        y: usize
      }

      let av = if let Some(bv) = map.get_mut(&x) {
        let after_value = bv.1;
        bv.1 = y;
        map.insert(y, (x, after_value));
        after_value
      } else {
        none
      };

      if let Some(rav) = map.get_mut(&av) {
        rav.0 = y;
      }

      continue
    } else {
      let (before, after) = map.remove(&x).unwrap_or((none,none));
      if let Some(bv) = map.get_mut(&before) {
        bv.1 = after;
      }
      if let Some(av) = map.get_mut(&after) {
        av.0 = before;
      }
    }
  }

  let mut que = VecDeque::new();
  let mut cv = 0;
  for (key, val) in &map {
    if val.0 == none {
      que.push_back(*key);
      cv = *key;
      break
    }
  }

  while let Some(&(_,av)) = map.get(&cv) {
    if av == none {
      break
    }
    que.push_back(av);
    cv = av;
  }

  println!("{}", que.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}