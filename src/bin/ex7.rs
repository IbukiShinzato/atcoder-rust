fn main() {
    let a = true;
    let b = false;
    let c = true;

    assert!(a == true || a == false);
    assert!(b == true || b == false);
    assert!(c == true || c == false);

    if a {
        print!("At");
    } else {
        print!("Yo");
    }

    if !a & b {
        print!("Bo");
    } else {
        print!("Co");
    }

    if a & b & c {
        print!("foo!");
    } else if true & false {
        print!("year!");
    } else if !a | c {
        print!("der");
    }

    println!("");
}
