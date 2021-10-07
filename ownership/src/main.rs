//s是对String的引用
fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) {
    some_str.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s //这里 s 离开作用域并被丢弃。其内存被释放
}

fn main() {
    /* 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    引用必须总是有效的。*/

    let mut str = String::from("hello");
    str.push_str(", world");

    println!("{}", str);

    //内存与分配

    //克隆
    // let clone_str = str.clone();
    // println!("{}", clone_str);

    //引用与借用
    // let str = String::from("hello");
    // let len = calculate_len(&str);
    //
    // println!("str: {}, len: {}", str, len);

    //可变引用
    // let mut str = String::from("hello");
    // change(&mut str);
    //
    // println!("str: {}", str);

    //悬垂引用（Dangling References）
    // let result = no_dangle();
    // println!("result: {}", result);
}
