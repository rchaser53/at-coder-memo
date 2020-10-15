use proconio::input;
// use nalgebra::min;
 
fn main() {
  input! {
    n: usize,
    k: usize,
    mut a: [usize;n],
    mut f: [usize;n]
  }

  if a.iter().sum::<usize>() <= k {
    println!("0");
    return
  }
  
  a.sort();
  f.sort();
  let (mut l, mut r) = (0, std::usize::MAX);
  while l + 1 < r {
    let mid = (l + r) / 2;
    let mut temp = 0;
    for i in 0..n {
      temp += a[i].checked_sub(mid / f[n-i-1]).unwrap_or(0);
    }
    if temp > k {
      l = mid;
    } else {
      r = mid;
    }
  }
  println!("{}", r);
}