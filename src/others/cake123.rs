use proconio::input;

fn main() {
  input! {
    x: usize,
    y: usize,
    z: usize,
    k: usize,
    mut xs: [usize;x],
    mut ys: [usize;y],
    mut zs: [usize;z]
  }
  
  let mut doubles: Vec<usize> = vec![0;x*y];
  for i in 0..x {
    for ii in 0..y {
      doubles[i*y + ii] = xs[i] + ys[ii];
    }
  }
  doubles.sort();
  doubles.reverse();

  let mut result: Vec<usize> = vec![0;z*k];  
  let kk = if doubles.len() < k {
    doubles.len()
  } else {
    k
  };
  
  for i in 0..kk {
    for ii in 0..z {
      result[i*z+ii] = doubles[i] + zs[ii];
    }
  }
  result.sort();
  result.reverse();
  
  for i in 0..k {
    println!("{}", result[i]);
  }
}