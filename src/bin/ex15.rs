use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
        p: [usize; n],
    }

    let ans = a
        .iter()
        .flat_map(|x| p.iter().map(move |y| (x, y)))
        .filter(|(x, y)| *x + *y == s)
        .count();

    println!("{}", ans);
}
