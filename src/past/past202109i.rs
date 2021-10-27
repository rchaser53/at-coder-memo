/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    vals: [usize;n]
  }

  let mut all_map = BTreeMap::new();
  let mut even_map = BTreeMap::new();
  for v in vals {
    *all_map.entry(v).or_insert(0) += 1;
    if v % 2 == 0 {
      *even_map.entry(v).or_insert(0) += 1;
    }
  }

  while !even_map.is_empty() {
    // a. 2の倍数の値をeven_mapから取り除き、2の倍数なら再度加える
    let (&cv, &num) = even_map.iter().next().unwrap();
    if num == 1 {
      even_map.remove(&cv);
    } else {
      even_map.insert(cv, num-1);
    }
    let nv = cv / 2;
    if nv % 2 == 0 {
      *even_map.entry(nv).or_insert(0) += 1;
    }

    // b. aで取得した値をall_mapから取り除き、1/2した値を加える
    let all_num = *all_map.get(&cv).unwrap();
    if all_num == 1 {
      all_map.remove(&cv);
    } else {
      all_map.insert(cv, all_num-1);
    }
    *all_map.entry(nv).or_insert(0) += 1;

    // c. 最小の値を取り除き、その値を3倍した値を加える
    let (&min_v, &min_num) = all_map.iter().next().unwrap();
    let nv = min_v * 3usize;
    if min_num == 1 {
      all_map.remove(&min_v);
    } else {
      all_map.insert(min_v, min_num-1);
    }
    *all_map.entry(nv).or_insert(0) += 1;

    // d. 最小の値が2の倍数の場合、even_mapから取り除き、その値を3倍した値を加える
    if min_v % 2 == 0 {
      if min_num == 1 {
        even_map.remove(&min_v);
      } else {
        even_map.insert(min_v, min_num-1);
      }
      *even_map.entry(nv).or_insert(0) += 1;
    }
  }

  println!("{}", all_map.keys().next().unwrap());
}