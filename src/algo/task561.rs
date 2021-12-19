/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
  use std::cmp::Reverse;
  
  fn readln<T: std::str::FromStr>() -> T {
    let mut tmp = String::new();
    std::io::stdin().read_line(&mut tmp).ok();
    tmp.trim().parse().ok().unwrap()
  }
  
  fn readvec<T: std::str::FromStr>() -> Vec<T> {
    readln::<String>()
        .split_whitespace()
        .map(|x| x.parse().ok().unwrap())
        .collect()
  }

// グループのsizeが必要な場合の union_find
fn size(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = find(parents, i) as usize;
  -1 * parents[ii]
}

fn find(parents: &mut Vec<isize>, i: isize) -> isize {
  let ii = i as usize;
  if parents[ii] < 0 {
    i
  } else {
    parents[ii] = find(parents, parents[ii]);
    parents[ii]
  }
}

fn connect(
  parents: &mut Vec<isize>,
  ws: &mut Vec<usize>,
  a: isize,
  b: isize
) -> bool {
  let mut pa = find(parents, a);
  let mut pb = find(parents, b);
  
  if pa == pb { return false }
  
  if size(parents, pa) < size(parents, pb) {
    let temp = pa;
    pa = pb;
    pb = temp;
  }
  
  let paa = pa as usize;
  let pbb = pb as usize;
  parents[paa] += parents[pbb];
  parents[pbb] = pa;

  ws[paa] += ws[pbb];
  
  true
}
  
  fn main() {
    let vals: Vec<usize> = readvec();
    let (n,q) = (vals[0], vals[1]);
    let mut ws:Vec<usize> = readvec();
    let mut parents = vec![-1;n];
    for _ in 0..q {
      let vals: Vec<usize> = readvec();
      let (t, x, y) = (vals[0], vals[1] as isize, vals[2] as isize);
      let xp = find(&mut parents, x);
      if t == 0 {
        connect(&mut parents, &mut ws, x, y);
      } else {
        if xp < 0 {
          println!("{}", ws[x as usize]);
        } else {
          println!("{}", ws[xp as usize]);
        }
      }
    }
    
  }