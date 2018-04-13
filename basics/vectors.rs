fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    
    let first = v[0];
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first in {}", first);
    println!("maybe_first is {:?}", maybe_first);

    let arr = [10,20,30];
    for i in arr.iter() {
        println!("{}", i);
    }
}
