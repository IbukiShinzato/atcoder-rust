use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut v = vec![vec!['-'; n]; n];

    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }

        let a = a - 1;
        let b = b - 1;

        v[a][b] = 'o';
        v[b][a] = 'x';
    }

    for row in v {
        println!(
            "{}",
            row.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
