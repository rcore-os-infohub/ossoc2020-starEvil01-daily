fn main() {
    let y = while_true(5);
    println!("y: {}", y);
    assert_eq!(y, 6);
}

fn while_true(x: i32) -> i32 {
    while true {
        return x+1;
    }
    //一下代码并不会执行
    println!("x: {}", x);
    //添加x返回,是为了补充while没有返回值的情况
    x
}
