/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

struct Helper {
  memo: Vec<Option<isize>>,
}

impl Helper {
  fn culc(&mut self, v:usize, mut now:isize) -> isize {
    if let Some(v) = self.memo[v] {
      return v
    }

    let mut count = 0;
    let mut i = 0;
    while 2usize.pow(i) <= v {
      if v >> i & 1 == 1 {
        count += 1;
      }
      i += 1;
    }

    if count == 0 {
      self.memo[v] = Some(0);
      return 0
    }

    let reminder = v % count;
    now += 1;
    if reminder == 0 {
      self.memo[v] = Some(now);
      now
    } else {
      let result = now + self.culc(reminder, 0);
      self.memo[v] = Some(result);
      result
    }
  }
}

fn repeat_square(n:isize, p:isize, m:isize) -> isize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2, m).pow(2) % m 
  } else {
    n * repeat_square(n, p-1, m) % m
  }
}

pub fn main(
) {
    input! {
      n:usize,
      s:Chars
    }

    let ni = n as isize;
    let max = 2usize * 10usize.pow(5) + 10;
    let memo = vec![None;max];
    let mut helper = Helper { memo };

    for i in 0..max {
      helper.culc(i, 0);
    }

    let mut count = 0;
    for i in 0..n {
      if s[i] == '1' {
        count += 1;
      }
    }
    
    let minus = if 1 < count {
      let mut temp = 0;
      for i in 0..n {
        if s[i] == '1' {
          temp += repeat_square(2, (n-i-1) as isize, count-1);
        }
        temp %= count-1;
      }
      temp
    } else {
      0
    };
    let mut plus = 0;
    for i in 0..n {
      if s[i] == '1' {
        plus += repeat_square(2, (n-i-1) as isize, count+1);
      }
      plus %= count+1;
    }
    
    for i in 0..n {
      if s[i] == '1' {
        if count <= 1 {
          println!("0");
          continue
        }

        let minus_mod = count-1;
        let mut first = minus;
        first -= repeat_square(2, (n-i-1) as isize, minus_mod);
        first %= minus_mod;
        first += minus_mod;
        first %= minus_mod;
        println!("{}", helper.memo[first as usize].unwrap() + 1);
      } else {
        let plus_mod = count+1;
        let mut first = plus;
        first += repeat_square(2, (n-i-1) as isize, plus_mod);
        first %= plus_mod;
        println!("{}", helper.memo[first as usize].unwrap() + 1);
      }
    }
}
