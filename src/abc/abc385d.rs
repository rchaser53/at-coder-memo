/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    sx:isize,
    sy:isize,
    xy:[(isize,isize);n],
    dc:[(char,isize);m]
  }

  let mut cols = HashMap::new();
  let mut rows = HashMap::new();
  for &(x,y) in &xy {
    rows.entry(y).or_insert(vec![]).push((x,0));
  }

  for &(x,y) in &xy {
    cols.entry(x).or_insert(vec![]).push((y,0));
  }

  let mut i = 1isize;
  let mut start = (sx,sy);
  // println!("x:{} y:{}", start.0, start.1);
  for (c,d) in dc {
    let (lx,ly) = start;
    let (nx,ny,p) = if c == 'L' {
      (lx-d,ly,true)
    } else if c == 'R' {
      (lx+d,ly,true)
    } else if c == 'D' {
      (lx,ly-d,false)
    } else {
      (lx,ly+d,false)
    };

    if p {
      if lx < nx {
        rows.entry(ly).or_insert(vec![]).push((lx,-i));
        rows.entry(ly).or_insert(vec![]).push((nx,i));
      } else {
        rows.entry(ly).or_insert(vec![]).push((nx,-i));
        rows.entry(ly).or_insert(vec![]).push((lx,i));
      }


    } else {
      if ly < ny {
        cols.entry(lx).or_insert(vec![]).push((ly,-i));
        cols.entry(lx).or_insert(vec![]).push((ny,i));
      } else {
        cols.entry(lx).or_insert(vec![]).push((ny,-i));
        cols.entry(lx).or_insert(vec![]).push((ly,i));
      }
    }
    start = (nx,ny);

    i += 1;
  }

  let mut set = HashSet::new();
  for (y, mut row) in rows {
    row.sort_by(|a,b| {
      let v = a.0.cmp(&b.0);
      if v == Ordering::Equal {
        a.1.cmp(&b.1)
      } else {
        v
      }
    });

    // println!("{:?}", &row);

    let mut p = HashSet::new();
    for (x, t) in row {
      if t != 0 {
        let t = t.abs();
        if p.contains(&t) {
          p.remove(&t);
        } else {
          p.insert(t);
        }
      }

      if t == 0 && !p.is_empty() {
        set.insert((x,y));
      }
    }
  }

  for (x, mut col) in cols {
    col.sort_by(|a,b| {
      let v = a.0.cmp(&b.0);
      if v == Ordering::Equal {
        a.1.cmp(&b.1)
      } else {
        v
      }
    });

    let mut p = HashSet::new();
    for (y, t) in col {
      if t != 0 {
        let t = t.abs();
        if p.contains(&t) {
          p.remove(&t);
        } else {
          p.insert(t);
        }
      }

      if t == 0 && !p.is_empty() {
        set.insert((x,y));
      }
    }
  }
  // println!("{:?}", &set);

  println!("{} {} {}",start.0, start.1, set.len());
}