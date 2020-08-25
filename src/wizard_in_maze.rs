use proconio::input;
use proconio::marker::{Chars, Isize1};
use std::collections::{HashSet, VecDeque};

fn main() {
  input! {
    h: isize,
    w: isize,
    start: (Isize1, Isize1),
    end: (Isize1, Isize1),
    map: [Chars;h as usize]
  }

  let mut seen: HashSet<(isize, isize)> = HashSet::new();
  let mut deque: VecDeque<(isize, isize, isize)> = VecDeque::new();
  deque.push_back((start.1, start.0, 0));
  
  while !deque.is_empty() {
    let (x, y, point) = deque.pop_front().unwrap();
    if x == end.1 && y == end.0 {
      println!("{}", point);
      return
    }

    if seen.contains(&(x, y)) {
      continue
    }    
    seen.insert((x, y));
    
    for r in -2..=2isize {
      for c in -2..=2isize {
        if r == 0 && c == 0 {
          continue
        }
        
        let new_x = x + c;
        let new_y = y + r;
        if new_x < 0 || new_x >= w
          || new_y < 0 || new_y >= h {
          continue
        }
        
        if map[new_y as usize][new_x as usize] != '.' {
          continue
        }

        if seen.contains(&(new_x, new_y)) {
          continue
        }
        
        if r.abs() + c.abs() == 1 {
          deque.push_front((new_x, new_y, point));
        } else {
          deque.push_back((new_x, new_y, point+1));
        }
      }
    }  
  }
  
  println!("-1");
}