use std::usize;

use proconio::input;

fn num_report(children: &Vec<Vec<usize>>, x: usize) -> usize {
    1 + children[x]
        .iter()
        .map(|&child| num_report(children, child))
        .sum::<usize>()
}

fn main() {
    input! {
        n: usize,
        mut p: [usize; n-1],
    }

    p.insert(0, 0);

    let mut children = vec![vec![]; n];
    for i in 1..n {
        let parent = p[i];
        children[parent].push(i);
    }

    for i in 0..n {
        let res = num_report(&children, i);
        println!("{}", res);
    }
}
