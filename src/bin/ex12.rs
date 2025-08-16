use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
    }

    let min_index = t
        .iter()
        .enumerate()
        .min_by_key(|&(_, val)| val)
        .map(|(i, _)| i + 1)
        .unwrap();

    println!("{}", min_index);
}
