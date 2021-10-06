fn main() {
    let mut str = String::from("hello");
    str.push_str(", world");

    println!("{}", str);

    //内存与分配
    //克隆
    let clone_str = str.clone();

    println!("{}", clone_str);
}
