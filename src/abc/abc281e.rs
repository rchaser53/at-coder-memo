/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
    a:[usize;n]
  }

  let mut result = vec![];
  let mut btreeset = BTreeSet::new();
  for i in 0..m {
    btreeset.insert((a[i], i));
  }

  let mut tot = 0;
  let mut temp_iter = btreeset.iter();
  let mut outset = BTreeSet::new();
  for _ in 0..k {
    // tot += temp_iter.next().unwrap_or(&(0,0)).0;
    tot += temp_iter.next().unwrap().0;
  }
  for _ in k..m {
    outset.insert(*temp_iter.next().unwrap());
  }
  result.push(tot);

  for v in &outset {
    btreeset.remove(&v);
  }

  // println!("{}", tot);
  // println!("{:?}", &outset);
  // println!("{:?}", &btreeset);

  for ii in 1..=n-m {
    let i = m+ii-1;
    let li = ii-1;
    let lv = (a[li], li);
    let cv = (a[i], i);

    // println!("i:{} li:{} == lv:{:?} cv:{:?}", i, li, lv, cv);
    // 古い値がbtreesetにある
    if btreeset.contains(&lv) {
      // 古い値を取り除く
      btreeset.remove(&lv);
      tot -= lv.0;

      outset.insert(cv);
      let min_v = *outset.iter().next().unwrap();
      outset.remove(&min_v);

      tot += min_v.0;
      btreeset.insert(min_v);
    }
    else {
      // 古い値を取り除く
      outset.remove(&lv);

      // 新しい値をoutsetに入れて、最小の値を取得する
      outset.insert(cv);
      let min_v = *outset.iter().next().unwrap();
      let max_v = *btreeset.iter().rev().next().unwrap();

      // 新しい値がoutsetの最小の値でbtreesetの最大値より小さい
      if min_v < max_v {
        tot -= max_v.0;
        tot += min_v.0;

        outset.remove(&min_v);
        btreeset.insert(min_v);
        btreeset.remove(&max_v);

        // outsetは別に幾つあっても問題ないはず
        outset.insert(max_v);
      }
    }

    result.push(tot);
  }

  println!("{}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}