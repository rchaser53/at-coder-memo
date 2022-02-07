#[derive(Debug)]
enum Expr {
  Num(i32),
  PlusMinus(bool),
  MulDiv(bool),
  Paren(bool)
}

fn convert(a:char) -> Expr {
  match a {
    '+' => Expr::PlusMinus(true),
    '-' => Expr::PlusMinus(false),
    '*' => Expr::MulDiv(true),
    '/' => Expr::MulDiv(false),
    '(' => Expr::Paren(true),
    ')' => Expr::Paren(true),
    _ => Expr::Num((a as u8 - '0' as u8) as i32)
  }
}

fn culc(left:Expr, op:Expr, right:Expr) -> Expr {
  let lv = if let Expr::Num(num) = left {
    num
  } else {
    unreachable!()
  };

  let rv = if let Expr::Num(num) = right {
    num
  } else {
    unreachable!()
  };

  match op {
    Expr::PlusMinus(flag) => {
      if flag {
        Expr::Num(lv + rv)
      } else {
        Expr::Num(lv - rv)
      }
    },
    Expr::MulDiv(flag) => {
      if flag {
        Expr::Num(lv * rv)
      } else {
        Expr::Num(lv / rv)
      }
    },
    _ => unreachable!()
  }
}

fn eat(mut exps:VecDeque<Expr>) -> Expr {
  let mut new_exps = VecDeque::new();
  while let Some(c) = exps.pop_front() {
    match c {
      Expr::MulDiv(t) => {
        let l = new_exps.pop_back().unwrap();
        let r = exps.pop_front().unwrap();
        new_exps.push_back(culc(l, Expr::MulDiv(t), r));
      },
      _ => {
        new_exps.push_back(c);
      }
    }
  }

  exps = new_exps;
  let mut new_exps = VecDeque::new();
  while let Some(c) = exps.pop_front() {
    match c {
      Expr::PlusMinus(t) => {
        let l = new_exps.pop_back().unwrap();
        let r = exps.pop_front().unwrap();
        new_exps.push_back(culc(l, Expr::PlusMinus(t), r));
      },
      _ => {
        new_exps.push_back(c);
      }
    }
  }
  
  if new_exps.len() != 1 {
    unreachable!()
  }
  new_exps.pop_back().unwrap()
}

pub fn max_depth(s: String) -> i32 {
  let mut s = s.chars().collect::<VecDeque<char>>();
  let mut que = VecDeque::new();
  while let Some(c) = s.pop_front() {
    if c == ')' {
      let mut exps = VecDeque::new();
      while let Some(v) = que.pop_back() {
        if let Expr::Paren(_) = v {
          break
        } else {
          exps.push_back(v);
        }
      }

      que.push_back(eat(exps));
    } else {
      que.push_back(convert(c));
    }
  }

  if let Expr::Num(v) = eat(que) {
    v
  } else {
    unreachable!()
  }
}