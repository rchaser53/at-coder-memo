use proconio::input;
use proconio::marker::*;
use std::collections::*;

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
  
  true
}

fn create_maze(
  i:usize
) -> Vec<Vec<usize>> {
  let mut maze = vec![vec![0;6];6];
  for j in 0..16 {
    if i >> j & 1 == 1 {
      let row = j / 4 + 1;
      let column = j % 4 + 1;
      maze[row][column] = 1;
    }
  }
  maze
}

fn judge(
  vals: &Vec<Vec<usize>>,
  maze: &mut Vec<Vec<usize>>,
) -> bool {
  let mut memo = vec![-1;36];
  let mut one_index = 0;

  for i in 0..5 {
    connect(&mut memo, i, i+1);
    connect(&mut memo, 30+i, 30+i+1);
    connect(&mut memo, i*6, (i+1)*6);
    connect(&mut memo, i*6+5, (i+1)*6+5);
  }

  for i in 1..=4 {
    for j in 1..=4 {
      let v = maze[i][j];
      let jv = j as isize;
      let kv = (i * 6) as isize;
      
      if vals[i-1][j-1] == 1 {
        one_index = jv + kv;
        if maze[i][j] == 0 {
          return false
        }
      }

      if maze[i-1][j] == v {
        connect(&mut memo, jv+kv, jv+kv-6);
      }

      if maze[i+1][j] == v {
        connect(&mut memo, jv+kv, jv+kv+6);
      }

      if maze[i][j+1] == v {
        connect(&mut memo, jv+kv, jv+kv+1);
      }

      if maze[i][j-1] == v {
        connect(&mut memo, jv+kv, jv+kv-1);
      }


    }
  }
  let zero_v = size(&mut memo, 0);
  let one_v = size(&mut memo, one_index);
  zero_v + one_v == 36
}

fn main() {
  input!{
    vals:[[usize;4];4]
  }

  let limit = 1 << 16;
  let mut count = 0usize;
  for i in 0..limit {
    let mut input = create_maze(i);
    if judge(&vals, &mut input) {
      count += 1;
    }
  }
  println!("{}", count);
}