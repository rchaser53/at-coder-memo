use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;
 
fn main() {
  input! {
    n: usize,
    k: usize,
    ps: [Usize1;n],
    cs: [isize;n]
  }
  
  let mut result = isize::min_value();
  for i in 0..n {
    let mut routes = vec![];
    let mut current_index = i;
    let mut total = 0;
    loop {
      routes.push(current_index);
      current_index = ps[current_index];
      total += cs[current_index];      
      if i == current_index {
         break
      }
    }
    
    let route_len = routes.len();
    let mut small_sum = 0;
    for ii in 0..route_len {
      small_sum += cs[routes[ii]];
      if ii + 1 > k { break }
      let s_result = if total > 0 {
        let times = (k - ii - 1) as isize / route_len as isize;
        small_sum + (total * times)
      } else {
        small_sum
      };
      result = std::cmp::max(result, s_result);
    }
  }
  println!("{}", result);
}