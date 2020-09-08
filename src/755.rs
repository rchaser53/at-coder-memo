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


use proconio::input;

struct Helper {
  pub count: usize,
  pub n: usize
}

impl Helper {
  fn traverse(&mut self, i: usize, v: usize) {
    if i == 10 {
      if v <= self.n && self.is_safe(v) {
        self.count += 1;
      }
      return
    }
      
    self.traverse(i+1, v+ 10usize.pow(i as u32)*7);
    self.traverse(i+1, v+ 10usize.pow(i as u32)*5);
    self.traverse(i+1, v+ 10usize.pow(i as u32)*3);

    if v <= self.n && self.is_safe(v) {
      self.count += 1;
    }
  }
  
  fn is_safe(&self, mut val: usize) -> bool {
    let mut f3 = false;
    let mut f5 = false;
    let mut f7 = false;
    
    while 0 < val {
      let v = val % 10;
      if v == 3 {
        f3 = true;
      } else if v == 5 {
        f5 = true;
      } else if v == 7 {
        f7 = true;
      } else {
        f3 = false;
        break
      }
      val /= 10;
    }
    
    f3 && f5 && f7
  }
}

fn main() {
  input! {
    n: usize,
  }

  let mut helper = Helper { count: 0, n };
  helper.traverse(0, 0);
  println!("{}", helper.count);
}