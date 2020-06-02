#[warn(non_snake_case)]
fn main() {
    let input = "3
101 9901 999999000001";

    culc(input);
}

fn culc(input: &str) {
    if let Some(val) = helper(input) {
        println!("{}", val);
    } else {
        println!("-1");
    }
}

fn helper(input: &str) -> Option<u64> {
    let input: Vec<&str> = input.split("\n").collect();
    let T: Vec<u64> = input[1]
        .split(" ")
        .map(|val| val.parse::<u64>().unwrap())
        .collect();

    for t in T.clone() {
        if t == 0 {
            return Some(0);
        }
    }

    let mut result = 1;
    let limit = u64::pow(10, 18);
    for t in T {
        result = result * t;
        if result > limit {
            return None;
        }
    }

    Some(result)
}
