/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    mut a:[usize;n],
    ix:[(Usize1,usize);q]
  }
  
  let mut btreeset = (0..=2*10usize.pow(5)+10).into_iter().collect::<BTreeSet<usize>>();
  let mut map = HashMap::new();
  for &v in &a {
    *map.entry(v).or_insert(0) += 1;
    btreeset.remove(&v);
  }

  for (i, nv) in ix {
    let cv = a[i];
    let entry = map.entry(cv).or_insert(0);
    if *entry == 1 {
      map.remove(&cv);
      btreeset.insert(cv);
    } else {
      *entry -= 1;
    }

    a[i] = nv;
    let entry = map.entry(nv).or_insert(0);
    if *entry == 0 {
      btreeset.remove(&nv);
    }
    *entry += 1;

    println!("{}", btreeset.iter().next().unwrap());
  }
}