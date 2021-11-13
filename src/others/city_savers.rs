use proconio::input;
 
fn main() {
  input! {
    n: usize,
    ais: [isize; n+1],
    bis: [isize; n]
  }
  
  let mut left = 0;
  let mut sum = 0;
  for i in 0..n {
    let a_val = if ais[i] - left > 0 {
      sum += left;
      ais[i] - left
    } else {
      sum += ais[i];
      0
    };
    let b_val = bis[i];

    if a_val < b_val {
      left = b_val - a_val;
      sum += a_val;
    } else {
      sum += b_val;
      left = 0;
    };
  }
  
  let last = if ais[n] - left > 0 {
    left
  } else {
    ais[n]
  };
  
  sum += last;
  println!("{}", sum);
}