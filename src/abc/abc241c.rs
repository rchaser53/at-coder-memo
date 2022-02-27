/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;


fn main() {
  input! {
    n:usize,
    rows:[Chars;n]
  }


  let mut max = 0;
  for i in 0..n {
    let mut temp = 0;
    for j in 0..6 {
      if rows[i][j] == '#' {
        temp += 1;
      }
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      if rows[i][j] == '#' {
        temp += 1;
      }
      if rows[i][j-6] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }

  for i in 0..n {
    let mut temp = 0;
    for j in 0..6 {
      if rows[j][i] == '#' {
        temp += 1;
      }
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      if rows[j][i] == '#' {
        temp += 1;
      }
      if rows[j-6][i] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }

  for i in 0..n {
    // rows[i][0]
    let mut success = true;
    let mut temp = 0;
    for j in 0..6 {
      let ni = i + j;
      if n <= ni {
        success = false;
        break
      }
      if rows[ni][j] == '#' {
        temp += 1;
      }
    }
    if !success {
      continue
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      let ni = i + j;
      if n <= ni { break }
      if rows[ni][j] == '#' {
        temp += 1;
      }
      if rows[ni-6][j-6] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }


  for i in 0..n {
    // rows[0][i]
    let mut success = true;
    let mut temp = 0;
    for j in 0..6 {
      let ni = i + j;
      if n <= ni {
        success = false;
        break
      }
      if rows[j][ni] == '#' {
        temp += 1;
      }
    }
    if !success {
      continue
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      let ni = i + j;
      if n <= ni { break }
      if rows[j][ni] == '#' {
        temp += 1;
      }
      if rows[j-6][ni-6] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }

  for i in 0..n {
    // rows[i][0]
    let mut temp = 0;
    let mut success = true;
    for j in 0..6 {
      if i < j {
        success = false;
        break
      }
      let nri = i - j;
      if rows[nri][j] == '#' {
        temp += 1;
      }
    }
    if !success {
      continue
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      if i < j { break }
      let nri = i - j;
      if rows[nri][j] == '#' {
        temp += 1;
      }
      if rows[nri+6][j-6] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }

  for i in 0..n {
    // rows[n-1][i]
    let mut temp = 0;
    let mut success = true;
    for j in 0..6 {
      let nci = i + j;
      let nri = n-1-j;
      if n <= nci {
        success = false;
        break
      }
      if rows[nri][nci] == '#' {
        temp += 1;
      }
    }
    if !success {
      continue
    }
    max = std::cmp::max(max, temp);

    for j in 6..n {
      let nci = i + j;
      let nri = n-1-j;
      if n <= nci { break }
      if rows[nri][nci] == '#' {
        temp += 1;
      }
      if rows[nri+6][nci-6] == '#' {
        temp -= 1;
      }
      max = std::cmp::max(max, temp);
    }
  }

  if 4 <= max {
    println!("Yes");
  } else {
    println!("No");
  }
}