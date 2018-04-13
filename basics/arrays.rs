fn sum(values: &[i32]) -> i32 {
    let mut res = 0;

    for i in 0..values.len() {
        res += values[i];
    }

    res
}

fn main() {
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    let res = sum(&arr);

    println!("first {}", first);
    println!("sum {}", res);

    let ints = [1,2,3];
    let floats = [1.1,2.1,3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1,2],[10,20]];

    // debug prints with {:?}
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);

    // slices and Option type
    let slice = &arr;
    let one = slice.get(0);
    let five = slice.get(5);

    println!("first {} {}", one.is_some(), one.is_none());
    println!("last {} {}", five.is_some(), five.is_none());
}
