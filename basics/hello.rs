fn main() {

    let answer = 42;

    // The '!' indicates that this is a macro call
    println!("Hello {}", answer);

    assert_eq!(answer, 42);

    // looping
    let mut sum = 0;    // variables are inherentyle immutable

    for i in 0..5 {
        sum += i;
    }

    println!("Sum: {}", sum);
}
