#[warn(non_snake_case)]
fn main() {
    let input = "9 45000";
    culc(input);
}

fn culc(input: &str) {
    println!("{}", helper(input));
}

fn helper(input: &str) -> String {
    let input: Vec<u64> = input
        .split(" ")
        .map(|val| val.parse::<u64>().unwrap())
        .collect();
    let N = input[0];
    let Y = input[1];

    for i in 0..N {
        for j in 0..(N - i) {
            let k = N - i - j;
            if k * 10000 + j * 5000 + i * 1000 == Y {
                return format!("{} {} {}", k, j, i);
            }
        }
    }

    format!("-1 -1 -1")
}
