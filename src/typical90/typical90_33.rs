use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
    // #[rustfmt::skip] source: proconio::source::line::LineSource<std::io::BufReader<&[u8]>>,
) {
    input! {
      // from source,    // NEED TO BE COMMENT OUT WHEN SUBMIT
      h:usize,
      w:usize
    }
    if h == 1 {
      println!("{}", w);
      return
    } else if w == 1 {
      println!("{}", h);
      return
    }

    let mut row_num = h/2;
    if h % 2 == 1 {
      row_num += 1;
    }
    let mut column_num = w/2;
    if w % 2 == 1 {
      column_num += 1;
    }

    println!("{}", row_num * column_num);
}
