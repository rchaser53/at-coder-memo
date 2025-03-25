/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    r:isize,
    c:isize,
    s:Chars
  }

  let mut map = HashMap::new();
  map.insert('N',(-1,0));
  map.insert('W',(0,-1));
  map.insert('S',(1,0));
  map.insert('E',(0,1));
  
  let tr = r;
  let tc = c;

  let mut cx = 0;
  let mut cy = 0;
  let mut result = vec![];
  let mut set = HashSet::new();
  set.insert((cx,cy));
  for c in s {
    if let Some(&(ai,aj)) = map.get(&c) {
      cx += ai;
      cy += aj;

      set.insert((-1*cx, -1*cy));
    }

    let rr = tr - cx;
    let rc = tc - cy;
    if set.contains(&(rr,rc)) {
      result.push(format!("1"));
    } else {
      result.push(format!("0"));
    }
  }
  println!("{}", result.into_iter().collect::<String>());
}