use proconio::input;

fn main() {
  input! {
    n: usize,
  }

  let mut count = 0;
  let mut i = 0;
  let mut v = 357;
  while v <= n {
    let mut temp = v;
    let mut f3 = false;
    let mut f5 = false;
    let mut f7 = false;
    while 0 < temp {
      let vv = temp % 10;
      if vv == 7 {
        f7 = true;
      } else if vv == 5 {
        f5 = true;
      } else if vv == 3 {
        f3 = true;
      } else {
        f3 = false;
        break
      }
      temp /= 10;
    }
    if f3 && f5 && f7 {
      count += 1;
    }
    
    if i % 3 == 0 {
      v += 6;
    } else {
      v += 2;
    }
    i += 1;
  }
  println!("{}", count);
}