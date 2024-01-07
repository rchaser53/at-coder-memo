/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:isize,
    q:usize,
  }
  
  let mut que = (1..=n).into_iter().map(|v| (v,0)).collect::<VecDeque<(isize,isize)>>();
  let mut map = HashMap::new();
  map.insert('R', (1,0));
  map.insert('L', (-1,0));
  map.insert('U', (0,1));
  map.insert('D', (0,-1));
  for _ in 0..q {
    input! {
      t: usize
    }

    if t == 1 {
      input!{
        c:char
      }

      let (tx,ty) = map.get(&c).unwrap();
      let cv = (que[0].0 + tx, que[0].1 + ty);
      que.pop_back();
      que.push_front(cv);
      continue
    }

    input!{
      i: Usize1        
    }
    println!("{} {}", que[i].0, que[i].1)
  }
}