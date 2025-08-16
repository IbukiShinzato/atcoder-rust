use proconio::input;
use std::process::exit;

fn main() {
    input! {
        n: usize,
        mut a: isize,
    }

    for i in 0..n {
        input! {
            op: char,
            b: isize,
        }

        match op {
            '+' => a += b,
            '-' => a -= b,
            '*' => a *= b,
            '/' => {
                if b == 0 {
                    println!("error");
                    break;
                } else {
                    a /= b;
                    if a < 0 {
                        a -= 1;
                    }
                }
            }
            _ => {
                println!("error");
                exit(1);
            }
        }

        println!("{} {}", i + 1, a);
    }
}
