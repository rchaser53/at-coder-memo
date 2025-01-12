/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut btset = BTreeSet::new();
  btset.insert((a[0], 0));

  let mut result = vec![0;n];
  for i in 1..n {
    let v = a[i];
    let mut add = 0;
    loop {
      if btset.is_empty() {
        btset.insert((v+i+add,i));
        break
      }
      let (num,ci) = btset.iter().next().unwrap();
      if i < *num {
        let len = btset.len();
        btset.insert((v+i+len+add,i));
        break
      } else if i == *num {
        add += 1;
      }
      btset.remove(&(*num,*ci));
    }
  }
  
  for (v, ti) in btset {
    let rv = v - ti - (n - 1 - ti);
    result[ti] = rv;
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}