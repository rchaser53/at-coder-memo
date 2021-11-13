use proconio::input;

const MOD: i64 = 1_000_000_007;

fn get_val(a: i64, b:i64) -> i64 {
  let c = get_pn(a + b);
  let p = get_pn(a) % MOD * get_pn(b) % MOD;
  c * modinv(p, MOD) % MOD
}
  
fn get_pn(v: i64) -> i64 {
  let mut result = 1;
  for i in 1..=v {
    result = result * i % MOD;
  }
  result
}

fn swap(a: i64, b: i64) -> (i64, i64) {
  (b, a)
}

fn modinv(
  mut a: i64,
  m: i64
) -> i64 {
    let mut b = m;
    let mut u = 1;
    let mut v = 0;
    while 0 < b {
      let t = a / b;
      a -= t * b;
      let s = swap(a, b);
      a = s.0;
      b = s.1;
      u -= t * v;
      let s = swap(u, v);
      u = s.0;
      v = s.1;
    }
    u %= m; 
    if u < 0 {
      u += m;
    }
    u
}

fn main() {
  input! {
    x: i64,
    y: i64,
  }
  
  let a = 2 * x - y;
  if a % 3 != 0 {
    println!("0");
    return
  }
  let a = a / 3;
  let b =  x - (2 * a);
  
  if a < 0 || b < 0 {
    println!("0");
    return
  }
  
  println!("{}", get_val(a, b));
}