use proconio::input;

fn main() {
    input! {
        a: usize,
        op: char,
        b: usize,
    }

    match op {
        '+' => println!("{}", a + b),
        '-' => println!("{}", a - b),
        '*' => println!("{}", a * b),
        '/' => {
            if b == 0 {
                println!("error");
            } else {
                println!("{}", a / b);
            }
        }
        _ => println!("error"),
    }
}
