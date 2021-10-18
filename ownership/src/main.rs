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

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    /* 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    引用必须总是有效的。*/

    // let mut str = String::from("hello");
    // str.push_str(", world");
    //
    // println!("{}", str);

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

    //Slice 类型

    // let mut s = String::from("hello");
    // let word = first_word(&s); //5

    // s.clear(); // 清空字符串

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！

    let s = String::from("hello world");

    let hello = &s[0..5]; //对部分 String 的引用
    let world = &s[6..11];

    let slice = &s[0..2]; //let slice = &s[..2]; 相等

    /*
    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
        */

    let result = first_word(&s);

    println!("result: {}", result);
}
