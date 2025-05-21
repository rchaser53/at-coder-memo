/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    n:usize,
    yx: [(Usize1, Usize1); n],
    q:usize,
    query: [(usize, Usize1); q],
  }
  
  let mut rows = vec![HashSet::new(); h];
  let mut cols = vec![HashSet::new(); w];
  for (y, x) in yx {
    rows[y].insert(x);
    cols[x].insert(y);
  }

  for (t,v) in query {
    if t == 1 {
    let y = v;
    let set = &rows[y];
    println!(
      "{}", set.len()
    );
    
    for x in set {
      cols[*x].remove(&y);
    }
    rows[y].clear();
    } else {
      let x = v;
      let set = &cols[x];
      println!(
        "{}", set.len()
      );
      
      for y in set {
        rows[*y].remove(&x);
      }
      cols[x].clear();
    }
  }
}