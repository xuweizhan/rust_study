fn main() {
    // 字面量
    literal();
    // 数组、元组
    arr();
    // 函数
    let f = function(5);
    println!("function -> {}", f);
    // 控制流： if else
    ctl_fun();
    // 循环体 loop while range 翻转
    for_fun();
    //
}

/**
 * 循环体
 */
fn for_fun() {
    // loop
    let mut loop_ctl = 0;
    // loop可以返回值， break后面接返回的值
    let loop_result = loop {
        loop_ctl = loop_ctl + 1;
        println!("for_fun -> loop 执行中{}!", loop_ctl);
        if loop_ctl == 3 {
            break loop_ctl * 10;
        };
    };
    println!("for_fun -> loop结果：{}", loop_result);

    //while
    let mut while_ctl = 3;
    while while_ctl != 0 {
        println!("for_fun -> while 执行中{}!", while_ctl);
        while_ctl = while_ctl - 1;
    }

    // for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("for_fun -> for 执行中, 元素为：{}", element);
    }

    // range
    // rev是翻转
    let ranges = (1..4).rev();
    for number in (1..4).rev() {
        println!("for_fun -> range -> {}!", number);
    }
    println!("{:?}", ranges);
}

/**
 * if else
 */
fn ctl_fun() {
    let number = 3;
    if number != 0 {
        println!("ctl_fun -> number 不为0");
    } else {
        println!("ctl_fun -> number 的值是{}", number);
    }
    let a = if number == 3 { number } else { 0 };
    println!("ctl_fun -> {}", a);
    // 分割
    println!("----------------------------");
}

/**
 * 函数
 */
fn function(x: i32) -> i32 {
    // 分割
    x + 1
    // x:        形参
    // ->:       代表定义返回值类型
    // x + 1:    不带分号代表着为返回值，带了分号为语句，返回()空元组
}

/**
 * 数组、元组、
 */
fn arr() {
    // 元组整数默认是i32, 与python不同，rust的元组是允许重复的
    // let tup = (500, 6.4, 1);
    let tup: (i64, f64, i32, i32) = (50000000000000, 6.4, 1, 1);
    let (x, y, z, j) = tup;
    println!("arr -> {}, {}, {}, {}", x, y, z, j);
    println!("arr -> {}, {}, {}, {}", tup.0, tup.1, tup.2, tup.3);

    // 数组
    //  -> 数组的长度是不允许发生改变的 且 必须是相同类型的值
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [5; 7];
    println!("arr -> {}", months[0]);
    println!("arr -> {}", a[0]);
    println!("arr -> {}", b[2]);

    // 分割
    println!("----------------------------");
}

/**
 * 字面量
 */
fn literal() {
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // arch	isize	usize
    let a = false;
    let b = 'z';
    // 单引号为单字节char
    // 双引号为文本
    println!("literal -> {}, {}", a, b);

    // 分割
    println!("----------------------------");
}
