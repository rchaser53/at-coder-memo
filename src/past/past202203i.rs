/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(map1:HashMap<isize,Vec<isize>>, map2:HashMap<isize,Vec<isize>>, n:usize) -> bool {
  let mut dict1 = HashMap::new();
  let mut dict2 = HashMap::new();
  let mut dots = vec![];
  for (key, arr1) in map1 {
    if let Some(arr2) = map2.get(&key) {
      if arr1.len() != arr2.len() {
        return false
      }
      for v in arr1 {
        dots.push(v*2);
        *dict1.entry(v*2).or_insert(0) += 1;
      }
      for &v in arr2 {
        dots.push(v*2);
        *dict2.entry(v*2).or_insert(0) += 1;
      }
    } else {
      return false
    }
  }

  dots.sort();
  let hv = (dots[n] + dots[n-1]) / 2;
  for (v1, num1) in dict1 {
    let v2 = if v1 < hv {
      hv + (hv - v1).abs()
    } else {
      hv - (hv - v1).abs()
    };
    if let Some(&num2) = dict2.get(&v2) {
      if num1 == num2 {
        dict2.remove(&num2);
      } else {
        return false
      }
    } else {
      return false
    }
  }
  true
}

fn main() {
  input! {
    n:usize,
    mut xy1: [(isize,isize);n],
    mut xy2: [(isize,isize);n]
  }

  xy1.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      a.1.cmp(&b.1)
    } else {
      v
    }
  });
  xy2.sort_by(|a,b| {
    let v = a.0.cmp(&b.0);
    if v == Ordering::Equal {
      a.1.cmp(&b.1)
    } else {
      v
    }
  });

  let mut success = true;
  for i in 0..n {
    if xy1[i].0 != xy2[i].0 || xy1[i].1 != xy2[i].1 {
      success = false;
      break
    }
  }
  if success {
    println!("Yes");
    return
  }

  let mut map1 = HashMap::new();
  let mut map2 = HashMap::new();
  for &(x,y) in &xy1 {
    map1.entry(y).or_insert(vec![]).push(x);
  }
  for &(x,y) in &xy2 {
    map2.entry(y).or_insert(vec![]).push(x);
  }
  if helper(map1, map2, n) {
    println!("Yes");
    return
  }

  let mut map1 = HashMap::new();
  let mut map2 = HashMap::new();
  for &(x,y) in &xy1 {
    map1.entry(x).or_insert(vec![]).push(y);
  }
  for &(x,y) in &xy2 {
    map2.entry(x).or_insert(vec![]).push(y);
  }

  if helper(map1, map2, n) {
    println!("Yes");
    return
  }
  println!("No");
}