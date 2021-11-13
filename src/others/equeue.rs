use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    vs: [isize;n]
  }
  
  let mut l_map: Vec<(isize, Vec<isize>)> = vec![];
  let mut sum = 0;
  let mut stack = vec![];
  l_map.push((0, vec![]));
  for i in 0..n {
    let v = vs[i];
    if v < 0 {
      stack.push(v);
      stack.sort();
    }
    sum += v;
    l_map.push((sum, stack.clone()));
  }

  let mut r_map: Vec<(isize, Vec<isize>)> = vec![];
  let mut sum = 0;
  let mut i = n - 1;
  let mut stack = vec![];
  r_map.push((0, vec![]));
  loop {
    let v = vs[i];
    if v < 0 {
      stack.push(v);
      stack.sort();
    }
    sum += v;
    r_map.push((sum, stack.clone()));
    
    if i == 0 { break }
    i -= 1;
  }
  
  let mut max = 0;
  for l in 0..=k {
    let r_max = k - l;
    for r in 0..=r_max {
      let mut remove_point = k - l - r;
      let (mut v, ms) = if n <= r + l {
        l_map[n].clone()
      } else {
        let left = l_map[l].clone();
        let right = r_map[r].clone();
        (left.0 + right.0, [left.1, right.1].concat())
      };
      
      for i in 0..remove_point {
        if ms.len() <= i { break }
        v -= ms[i];
      }
      max = std::cmp::max(max, v);
    }
  }
  println!("{}", max);
}