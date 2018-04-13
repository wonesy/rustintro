fn sqr(x: f64) -> f64 {
    x * x
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn mutates (x: &mut f64) {
    *x = 1.0
}

fn main() {
    let sqrres = sqr(2.0);
    let absres = abs(-1000.0);
    let i = 4;

    let res1 = by_ref(&i);
    let res2 = by_ref(&res1);

    let mut cameron = 45.0;

    println!("Square: {}", sqrres);
    println!("Absolute: {}", absres);

    println!("Refs: {} {} {}", i, res1, res2);

    mutates(&mut cameron);
    println!("Mutated: {}", cameron); 
}
