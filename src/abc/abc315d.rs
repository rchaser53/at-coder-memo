/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn helper(map: &HashMap<char, usize>) -> bool {
  if map.keys().len() > 1 { return false }
  if let Some(key) = map.keys().next() {
    if let Some(&num) = map.get(key) {
      num > 1
    } else {
      unreachable!()
    }
  } else {
    true
  }
}

fn main() {
  input! {
    h:usize,
    w:usize,
    mut rows:[Chars;h]
  }

  let mut rs = HashMap::new();
  let mut cs = HashMap::new();
  for i in 0..h {
    rs.insert(i, HashMap::new());
  }
  for i in 0..w {
    cs.insert(i, HashMap::new());
  }

  for i in 0..h {
    for j in 0..w {
      let c = rows[i][j];
      if let Some(map) = rs.get_mut(&i) {
        *map.entry(c).or_insert(0) += 1;
      }
      if let Some(map) = cs.get_mut(&j) {
        *map.entry(c).or_insert(0) += 1;
      }
    }
  }

  let mut dirty = true;
  while dirty {
    dirty = false;
    
    let mut r_memo = vec![];
    for (i, map) in &rs {
      if helper(map) {
        r_memo.push(*i);
        dirty = true;
      }
    }
    let mut r_target = HashMap::new();
    for i in r_memo {
      let map = rs.remove(&i).unwrap();
      if let Some(key) = map.keys().next() {
        *r_target.entry(*key).or_insert(0) += 1;
      }
    }

    let mut c_memo = vec![];
    for (i, map) in &cs {
      if helper(map) {
        c_memo.push(*i);
        dirty = true;
      }
    }
    let mut c_target = HashMap::new();
    for i in c_memo {
      let map = cs.remove(&i).unwrap();
      if let Some(key) = map.keys().next() {
        *c_target.entry(*key).or_insert(0) += 1;
      }
    }

    for (c, num) in r_target {
      for (_, map) in &mut cs {
        let entry = map.entry(c).or_insert(0);
        if *entry == num {
          map.remove(&c);
        } else {
          *entry -= num;
        }
      }
    }

    for (c, num) in c_target {
      for (_, map) in &mut rs {
        let entry = map.entry(c).or_insert(0);
        if *entry == num {
          map.remove(&c);
        } else {
          *entry -= num;
        }
      }
    }
  }
  
  if rs.keys().len() == 0 {
    println!("0");
  } else {
    println!("{}", rs.keys().len() * cs.keys().len());
  }
}