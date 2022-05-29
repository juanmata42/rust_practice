fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);
    // use isize
    let idx: usize = 0;
    a[idx] = 312;
    println!("a[0] = {}", a[idx]);
    //option
    match a.get(1) {
        Some(x) => println!("a[1] = {}", x),
        None => println!("a[6] is not defined"),
    }
}
fn main() {
    vectors();
}
