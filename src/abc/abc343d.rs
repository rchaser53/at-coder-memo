/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    t:usize,
    ab:[(Usize1,usize);t]
  }
  
  let mut map = HashMap::new();
  map.insert(0,n);
  let mut dict = vec![0;n];

  for (a,b) in ab {
    let cv = dict[a];
    let nv = cv + b;

    let entry = map.entry(cv).or_insert(0);
    if *entry == 1 {
      map.remove(&cv);
    } else {
      *entry -= 1;
    }
    dict[a] = nv;

    *map.entry(nv).or_insert(0) += 1;

    println!("{}", map.keys().len());
  }
}