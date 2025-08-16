use proconio::input;

fn main() {
    input! {
        a: [usize; 5]
    }

    if a.iter().zip(a.iter().skip(1)).any(|(x, y)| x == y) {
        println!("YES");
    } else {
        println!("NO");
    }
}
