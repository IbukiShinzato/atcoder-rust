use proconio::input;

fn f0(_: usize) -> usize {
    1
}

fn f1(n: usize, m: usize) -> usize {
    let mut s = 0;

    for _ in 0..n {
        s += 1;
    }

    for _ in 0..m {
        s += 1;
    }

    s
}

fn f2(n: usize) -> usize {
    let mut s = 0;

    for _ in 0..n {
        let mut t = n;
        let mut cnt = 0;

        while t > 0 {
            cnt += 1;
            t /= 2;
        }

        s += cnt;
    }

    s
}

fn f3(_: usize) -> usize {
    let mut s = 0;

    for _ in 0..3 {
        for _ in 0..3 {
            s += 1;
        }
    }

    s
}

#[allow(unused)]
fn f4(n: usize) -> usize {
    let mut s = 0;

    for i in 0..n {
        for j in 0..n {
            s += i + j;
        }
    }

    s
}

fn f5(n: usize, m: usize) -> usize {
    let mut s = 0;

    for i in 0..n {
        for j in 0..m {
            s += i + j;
        }
    }

    s
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    #[allow(unused)]
    let a0 = -1;
    #[allow(unused)]
    let a1 = -1;
    #[allow(unused)]
    let a2 = -1;
    #[allow(unused)]
    let a3 = -1;
    let a4 = -1;
    #[allow(unused)]
    let a5 = -1;

    let a0 = f0(n);
    let a1 = f1(n, m);
    let a2 = f2(n);
    let a3 = f3(n);
    // let a4 = f4(n);
    let a5 = f5(n, m);

    println!("f0: {}", a0);
    println!("f1: {}", a1);
    println!("f2: {}", a2);
    println!("f3: {}", a3);
    println!("f4: {}", a4);
    println!("f5: {}", a5);
}
