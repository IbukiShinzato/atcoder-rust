use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }

    let ans: Vec<String> = s.into_iter().filter(|x| x.len() >= k).collect();
    println!("{}", ans.join(" "));
}
