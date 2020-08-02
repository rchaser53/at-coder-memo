use proconio::input;
fn helper(mut v: usize) -> usize {
  let mut ret = 0;
  while v % 2 == 0 {
    v /= 2;
    ret += 1;
  }
  ret
}

fn gcd(a:usize, b: usize) -> usize {
  if (b == 0) {
    return a
  }
  gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
  a * b / gcd(a, b)
}

fn main() {
  input! {
    n: usize,
    mut m: usize,
    mut ais: [usize;n] 
  }
  
  for i in 0..ais.len() {
    if ais[i] % 2 == 1 {
      println!("0");
      return
    }
    ais[i] /= 2;
  }
  
  let t = helper(ais[0]);
  for i in 0..ais.len() {
    if helper(ais[i]) != t {
      println!("0");
      return
    }
    ais[i] >>= t;
  }
  m >>= t;

  let mut lcm_val = 1;
  for i in 0..ais.len() {
    lcm_val = lcm(lcm_val, ais[i]);
    if lcm_val > m {
      println!("0");
      return
    }
  }
    
  let mut result = m / lcm_val; 
  result = (result + 1) / 2;
  
  println!("{}", result);
}