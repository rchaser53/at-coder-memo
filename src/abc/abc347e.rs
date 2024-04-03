/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    x:[Usize1;q]
  }

  let mut set = HashSet::new();
  let mut memo = vec![0;q+1];
  let mut map = HashMap::new();
  for i in 0..q {
    let v = x[i];
    if set.contains(&v) {
      set.remove(&v);
    } else {
      set.insert(v);
    }

    memo[i+1] = memo[i];
    memo[i+1] += set.len();
    map.entry(v).or_insert(vec![]).push(i);
  }

  let mut a = vec![0;n];
  for (ti, mut arr) in map {
    if n <= ti { continue }
    arr.push(q);

    for i in 0..arr.len()-1 {
      if i % 2 == 1 { continue }
      let to = arr[i+1];
      let from = arr[i];
      a[ti] += memo[to] - memo[from];
    }
  }
  println!("{}", a.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}