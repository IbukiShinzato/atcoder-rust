use proconio::input;

fn sum_scores(scores: Vec<usize>) -> usize {
    scores.iter().sum::<usize>()
}

fn output(sum_a: usize, sum_b: usize, sum_c: usize) {
    println!("{}", sum_a * sum_b * sum_c)
}

fn input_vector(n: usize) -> Vec<usize> {
    input! {
        vec: [usize; n]
    }

    vec
}

fn main() {
    input! {
        n: usize
    }

    let a = input_vector(n);
    let b = input_vector(n);
    let c = input_vector(n);

    let sum_a = sum_scores(a);
    let sum_b = sum_scores(b);
    let sum_c = sum_scores(c);

    output(sum_a, sum_b, sum_c);
}
