# rCore-os

*七月*

| Mon | Tues | Wed | Thur | Fri | Sat | Sun |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|   |   | 1 | 2 | 3 | 4<br>([D1](#day1)) | 5<br>([D2](#day2)) |
| 6 | 7 | 8 | 9 |  10  | 11 | 12 |
| 13 | 14 | 15 | 16 | 17 | 18 | 19 |
| 20 | 21 | 22 | 23 | 24 | 25 | 26 |
| 27 | 28 | 29 | 30 | 31 |  |  |



## Day1

> 从上个月26号收到消息，就开始了断断续续的rust学习，昨天一看，突然发现其他同学们的效率与进度，压力倍增。今天开始正式挑战一下自己，加油！
>

### 事件1：继续学习剩余的《Rust程序设计语言》13~16章

[《Rust程序设计语言》](https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html )

- 结构体闭包

  >eg13-10的思路类似“单例模式”，控制复杂耗时的代码最多只被执行一次
  >
  >涉及参数不同的问题需要考虑是否需要更新cacher

- 闭包会捕获其环境

  > 联想到了python中的lambda表达式、ES6的箭头函数，有很多共同之处。

- 迭代器

  >- iter：生成一个不可变引用的迭代器
  >- into_iter：获取所有权并返回拥有所有权的迭代器
  >- iter_mut：迭代可变引用
  >
  >.map()：创建新的迭代器
  >
  >.collect()：消费迭代器

### 事件2： 开始小练习

[from_into](https://github.com/starEvil01/rCore-os/tree/master/rustlings/exercisesconversions/from_into.rs)

```
let age = match s.next() {
    Some(a) => a.parse::<usize>(),
    None => return Person::default(),
};
let mut age = match age {
    Ok(age) => age,
    Err(e) => return Person::default(),
};
```

> 这里应该有更简洁的写法，学习到了回来改


## Day2

### 事件1 学习剩余的《Rust程序设计语言》17~20章

[《Rust程序设计语言》](https://kaisery.github.io/trpl-zh-cn/ch18-02-refutability.html)

终于过完了所有语法，一些地方还没有消化。

### 事件2： 小练习

历经曲折，终于在凌晨12点完成所有小练习。

其中一些卡壳的地方还是因为接触的太少。

接下来的学习中再接再厉

## Day3

> 第三天