use proconio::input;

fn main() {
  input! {
    n: usize,
    mut vals: [usize;n]
  }
  
  let mut flag = true;
  let mut index = 0;
  let mut count = 0;
  while flag {
    flag = false;
    let mut sc = 0;
    for i in 0..n {
      if vals[i] != 0 {
        vals[i] -= 1;
        sc += 1;
        flag = true;
      } else {
        if 0 < sc {
          count += 1;
        }
        sc = 0;
      }
    }
    if 0 < sc {
      count += 1;
    }
  }
  
  println!("{}", count);
}