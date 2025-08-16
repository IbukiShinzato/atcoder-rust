use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let avg = a.iter().sum::<usize>() / n;

    for &ai in &a {
        println!("{}", avg.abs_diff(ai));
    }
}
