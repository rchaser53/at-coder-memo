use proconio::input;

fn main() {
  input! {
    a: f64,
    b: f64,
    h: f64,
    m: f64,
  };

  let long_angle =  m / 60f64;
  let short_angle = h / 12f64 + long_angle / 12f64;
  let angle = long_angle - short_angle;
  let rad = std::f64::consts::PI * 2.0 * angle;

  if angle == 0f64 {
    println!("{}", (a - b).abs());
  } else {
    let result = (a * a + b * b - 2f64 * a * b * rad.cos()).sqrt();
    println!("{}", result);
  }
}