use proconio::input;
 
fn main() {
  input! {
    n: usize,
    mut vals: [isize;n]
  }
  
  for i in 1..=n {
    vals[i-1] = vals[i-1] - (i as isize);
  }
  vals.sort();
  
  let targets = if n % 2 == 1 {
    vec![vals[n/2]]
  } else {
    vec![vals[n/2]-1, vals[n/2], vals[n/2]+1]
  };
  
  let mut min = isize::max_value();
  for v in targets {
    let mut temp = 0;
    for i in 0..n {
      temp += (vals[i] - v).abs();
    }
    min = std::cmp::min(min, temp);
  }
  
  println!("{}", min);
}