use proconio::input;
 
fn main() {
  input! {
    n: usize,
    mut k: usize,
    mut vals: [isize;n]
  }
  
  let mut min = isize::max_value();
  let mut index = n-1;
  loop {
    let i = index - k + 1;
    min = std::cmp::min(
      min,
      vals[index].abs() + (vals[index] - vals[i]).abs()
    );
    
    if i == 0 { break }
    index -= 1;
  }
  
  let mut index = 0;
  loop {
    let i = index + k-1;
    min = std::cmp::min(
      min,
      vals[index].abs() + (vals[index] - vals[i]).abs()
    );
    
    if n-1 <= i { break }
    index += 1;
  }
  println!("{}", min);
}