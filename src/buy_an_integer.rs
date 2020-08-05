use proconio::input;
 
fn output(v: i128) {
  let v = if v >= 10i128.pow(9) {
    10i128.pow(9)
  } else {
    v
  };
  println!("{}", v);
}
 
fn main() {
  input! {
    a: i128,
    b: i128,
    x: i128
  }
 
  if x >= (a * 1_000_000_000) + (10 * b) {
    output(1_000_000_000);
    return
  }
  
  let mut max = 0;
  for i in 1..10 {
    let temp = x - (b * i);
    if temp <= a {
      output(max);
      return
    }
 
    let val = temp / a;
    if val < i {
      output(max);
      return
    }
    
    let max_temp_val = 10i128.pow(i as u32);
    let val = if val >= max_temp_val {
      max_temp_val - 1
    } else {
      val
    };
    
    max = std::cmp::max(val, max);
  }
  
  output(max);
}