#[allow(unused_imports)]
use proconio::input;
 
fn main() {
  input! {
    h: usize,
    w: usize,
    mut vals: [String;h]
  }
  let mut map: Vec<Vec<char>> = vec![];
  for val in vals {
    map.push(val.chars().collect());
  }
  
  let mut row = map.len()-1;
  let mut column = map[0].len()-1;
  while 0 <= row {
    let mut flag = true;
    for c in map[row].iter() {
      if c == &'#' {
        flag = false;
          break
      }
    }
 
    if flag {
      map.remove(row);
      if map.is_empty() { break }
      row = map.len() - 1;
      continue
    }
 
    if row == 0 { break }
    row -= 1;
  }
  
  row = map.len();
  while 0 <= column {
    let mut flag = true;
    for r in 0..row {
      let c = map[r][column];
      if c == '#' {
        flag = false;
        break
      }
    }
 
    if flag {
      for r in 0..row {
        map[r].remove(column);
      }
      if map[0].is_empty() { break }
      column = map[0].len() - 1;
      continue
    }
 
    if column == 0 { break }
    column -= 1;
  }
  
  for v in map {
    println!("{}", v.iter().collect::<String>());
  }
}