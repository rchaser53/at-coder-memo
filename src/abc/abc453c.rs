use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let lengths: Vec<i64> = (0..n)
        .map(|_| it.next().unwrap().parse::<i64>().unwrap() * 2)
        .collect();

    let mut dp: HashMap<i64, i32> = HashMap::new();
    dp.insert(1_i64, 0_i32);

    for len in lengths {
        let mut next: HashMap<i64, i32> = HashMap::new();

        for (&dist, &score) in &dp {
            let away = dist + len;
            next.entry(away)
                .and_modify(|best: &mut i32| *best = (*best).max(score))
                .or_insert(score);

            let toward = (dist - len).abs();
            let add = if len > dist { 1 } else { 0 };
            next.entry(toward)
                .and_modify(|best: &mut i32| *best = (*best).max(score + add))
                .or_insert(score + add);
        }

        dp = next;
    }

    let answer = dp.values().copied().max().unwrap();
    println!("{answer}");
}
