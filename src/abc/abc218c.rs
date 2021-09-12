use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn turn(
  v: Vec<Vec<char>>
) -> Vec<Vec<char>> {
  let n = v.len();
  let mut result = vec![vec!['a';n];n];

  for i in 0..n {
    for j in 0..n {
      result[i][j] = v[n-1-j][i];
    }
  }
  result
}

fn main() {
  input!{
    n:usize,
    mut s:[Chars;n],
    m:[Chars;n]
  }

  let mut m_pos = (0, 0);
  let mut done = false;
  for i in 0..n {
    for j in 0..n {
      if m[i][j] == '#' {
        m_pos = (j, i);
        done = true;
        break
      }
    }
    if done {
      break
    }
  }

  let mut sc = 0;
  let mut mc = 0;
  for i in 0..n {
    for j in 0..n {
      if s[i][j] == '#' {
        sc += 1;
      }
      if m[i][j] == '#' {
        mc += 1;
      }
    }
  }
  if sc != mc {
    println!("No");
    return
  }

  for _ in 0..5 {
    let mut s_pos = (0, 0);
    let mut done = false;
    for i in 0..n {
      for j in 0..n {
        if s[i][j] == '#' {
          s_pos = (j, i);
          done = true;
          break
        }
      }
      if done {
        break
      }
    }

    let ax = (m_pos.0 - s_pos.0) as isize;
    let ay = (m_pos.1 - s_pos.1) as isize;
    let mut success = true;
    let inn = n as isize;

    for i in 0..n {
      let ii = i as isize;
      for j in 0..n {
        let ij = j as isize;
        if s[i][j] == '#' {
          let ri = ii + ay;
          let rj = ij + ax;
          if ri < 0 || inn <= ri {
            success = false;
            break
          }
          if rj < 0 || inn <= rj {
            success = false;
            break
          }

          if m[ri as usize][rj as usize] == '.' {
            success = false;
          }
        }
      }
    }

    if success {
      println!("Yes");
      return
    }
    s = turn(s);
  }
  println!("No");
}
