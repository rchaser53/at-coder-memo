/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    rows:[[usize;w];h],
    n:usize,
    t:[(Usize1,Usize1);n]
  }

  let mut columns = vec![VecDeque::new();w];

  for i in 0..w {
    for j in 0..h {
      columns[i].push_front(rows[j][i]);
    }

    for j in (0..h).rev() {
      if columns[i][j] > 0 {
        break
      }
      columns[i].pop_back();
    }
  }

  for (rr,c) in t {
    let cr = h - 1 - rr;
    if let Some(v) = columns[c].get_mut(cr) {
      *v = 0;

      loop {
        if let Some(&v) = columns[c].get(cr) {
          if v == 0 {
            columns[c].remove(cr);
          } else {
            break
          }
          
        } else {
          break
        }
      }
    }
  }
  // println!("{:?}", &columns);

  let mut result = vec![vec![0;w];h];
  for c in 0..w {
    for cr in 0..columns[c].len() {
      let rr = h - 1 - cr;
      result[rr][c] = columns[c][cr];
    }
  }

  for row in result {
    println!("{}", row.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
  }
}