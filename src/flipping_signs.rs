use proconio::input;
 
fn main() {
  input! {
    n: usize,
    vals: [isize;n]
  }
  
  let mut minus_count = 0;
  let mut total = 0;
  let mut min = isize::max_value();
  for v in vals.iter() {
    if v < &0 {
      minus_count += 1;
    }
    let abs_v = v.abs();
    min = std::cmp::min(min, abs_v);
    total += abs_v;
  }
  
  if minus_count % 2 == 0 {
    println!("{}", total);
  } else {
    println!("{}", total - 2 * min);
  } 
}