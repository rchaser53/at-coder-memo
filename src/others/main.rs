#[warn(non_snake_case)]
fn main() {
    let input = "erasedream";
    let input = "dreameraser";
    let input = "dreamerer";
    culc(input);
}

fn culc(input: &str) {
    println!("{}", helper(input));
}

fn helper(input: &str) -> bool {
    dbg!(input);
    if input.len() == 0 {
        return true;
    }

    if input.starts_with("dream") {
        if helper(&input[5..]) {
            return true;
        }
    }

    if input.starts_with("dreamer") {
        if helper(&input[6..]) {
            return true;
        }
    }

    if input.starts_with("erase") {
        if helper(&input[5..]) {
            return true;
        }
    }

    if input.starts_with("eraser") {
        if helper(&input[6..]) {
            return true;
        }
    }

    false
}
