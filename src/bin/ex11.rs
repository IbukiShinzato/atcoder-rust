use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 1;

    for i in (1..s.len()).step_by(2) {
        let cal = s[i];
        let one = s[i + 1].to_digit(9_u32 - 0_u32).unwrap() as isize;

        match cal {
            '+' => ans += one,
            '-' => ans -= one,
            _ => eprintln!("error"),
        }
    }

    println!("{}", ans);
}
