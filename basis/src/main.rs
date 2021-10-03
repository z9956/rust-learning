fn log(num: i32, str: char) {
    println!("num is :{}, str is :{}", num, str);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    /*
    不用加分号 x + 1;
    */
    x + 1
}

fn main() {
    // 控制流
    // 代码中的条件，必须是bool
    let num = 3;

    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let condition = true;
    let val = if condition { 5 } else { 6 };

    println!("val :{}", val);

    // let result = plus_one(5);
    // println!("result: {}", retult);

    // let x = five();
    // println!("x: {}", x);

    // let x = 5;
    //
    // let y = {
    //     let x = 3;
    //     x + 1;
    // };

    //函数
    // log(452, '😻');

    /*
    复合类型
    原生符合类型: tuple(元组) array(数组)
    */
    //tuple
    /*let tup: (i32, f64, u8) = (500, 6.4, 1);
    //模式匹配解构
    let (x, _, _) = tup;
    println!("the value of x is: {}", x);
    //点号（.）
    println!("tup 0 is: {}", tup.0);*/

    /*
    array
    array是固定长度
    */
    /* let arr = [1, 2, 3, 4, 5, 6];
    let list: [i32; 3] = [1, 2, 3];

    //变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同
    let data = [3; 2];

    let first = data[0];*/

    /*
    字符类型
    char类型由单引号指定
    char类型的大小为四个字节
    */
    // let str = 'z';
    // let cat = '😻';

    //数值运算
    // let sum = 5 + 10;
    // let difference = 95.5 - 4.2;
    // let product = 4 * 5;
    // let remainder = 43 % 5;

    //浮点型
    // let x = 2.0;
    // let y: f32 = 3.0;

    /* //data types
    let guess: u32 = "42".parse().expect("not a number");
    println!("guess: {}", guess);*/

    /*let mut x = 5;
    println!("the value of x is: {}", x);

    x = 6;
    println!("the value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("the value is :{}", THREE_HOURS_IN_SECONDS);

    let num = 5;
    let num = num * 3;

    //隐藏（Shadowing)
    {
        let num = num * 2;
        println!("16 num: {}", num);
    }

    println!("The value of num is :{}", num);*/
}
