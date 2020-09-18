use proconio::input;
use std::collections::VecDeque;

fn main() {
  input! {
    x: usize,
    y: usize,
    a: usize,
    b: usize,
    c: usize,
    mut pvs: [usize;a],
    mut qvs: [usize;b],
    mut rvs: [usize;c]
  }
  pvs.sort();
  pvs.reverse();
  qvs.sort();
  qvs.reverse();
  rvs.sort();
  rvs.reverse();
  
  let mut result = 0;

  let mut reds: VecDeque<usize> = vec![0;x].into_iter().collect();
  let mut greens: VecDeque<usize> = vec![0;y].into_iter().collect();
  
  for i in 0..x {
    reds[i] = pvs[i];
  }
  for i in 0..y {
    greens[i] = qvs[i];
  }
  
  while !rvs.is_empty() {
    let rs = reds[x-1];
    let gs = greens[y-1];
    let val = rvs.remove(0);
  
    if rs < gs {
      if val <= rs {
        break
      } else {
        reds.pop_back();
        reds.push_front(val);
      }
    } else {
      if val <= gs {
        break
      } else {
        greens.pop_back();
        greens.push_front(val);
      }
    }
  }
  
  let result = reds.iter().sum::<usize>() + greens.iter().sum::<usize>();
  println!("{}", result);
}
