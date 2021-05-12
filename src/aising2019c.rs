/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      h:usize,
      w:usize,
      rows:[Chars;h]
    }

    let mut map = vec![vec![0;w];h];
    let mut count = 0;
    for i in 0..h {
      for j in 0..w {
        let mut stack = vec![(i,j)];
        if map[i][j] != 0 || rows[i][j] == '#' { continue }
        count += 1;
        while !stack.is_empty() {
          let (r, c) = stack.pop().unwrap();
          if 0 < r && rows[r][c] != rows[r-1][c] && map[r-1][c] == 0 {
            map[r-1][c] = count;
            stack.push((r-1,c));
          }

          if r < h-1 && rows[r][c] != rows[r+1][c] && map[r+1][c] == 0 {
            map[r+1][c] = count;
            stack.push((r+1,c));
          }

          if 0 < c && rows[r][c] != rows[r][c-1] && map[r][c-1] == 0 {
            map[r][c-1] = count;
            stack.push((r,c-1));
          }

          if c < w-1 && rows[r][c] != rows[r][c+1] && map[r][c+1] == 0 {
            map[r][c+1] = count;
            stack.push((r,c+1));
          }
        }
      }
    }

    let mut dict = HashMap::new();
    for i in 0..h {
      for j in 0..w {
        let v = map[i][j];
        if v == 0 { continue }
        let mut entry = dict.entry(v).or_insert((0,0));
        entry.0 += 1;
        if rows[i][j] == '#' {
          entry.1 += 1;
        }
      }
    }

    let mut result = 0usize;
    for (_, (total, black)) in dict {
      let white = total - black;
      result += white * black;
    }
    println!("{}", result);
}
