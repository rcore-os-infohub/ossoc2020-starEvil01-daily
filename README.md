# rCore-os

*七月*

| Mon | Tues | Wed | Thur | Fri | Sat | Sun |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
|   |   | 1 | 2 | 3 | 4<br>([D1](#day1)) | 5<br>([D2](#day2)) |
| 6<br>([D3](#day3)) | 7<br>([D4](#day4)) | 8<br>([D5](#day5)) | 9<br/>([D6](#day6)) |  10  | 11 | 12 |
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

### 事件1： 学习剩余的《Rust程序设计语言》17~20章

[《Rust程序设计语言》](https://kaisery.github.io/trpl-zh-cn/ch18-02-refutability.html)

终于过完了所有语法，一些地方还没有消化。

### 事件2： 小练习

历经曲折，终于在凌晨12点完成所有小练习。

其中一些卡壳的地方还是因为接触的太少。

接下来的学习中再接再厉

## Day3

### 事件1：阅读《Rust编程之道》

#### 第二章

- non_snake_case：rust的包命名习惯，如果想使用驼峰命名，需要添加：` #![allow(non_snake_case)]`

  > struct的命名还是使用驼峰式命名规则

- 《Rust编程之道》，2-5的代码，书中提到代码第7行的错误已经不会再出现。

- 1..101：是一个Range类型，它是一个迭代器。

- 范围匹配 `1...3` 现在的版本建议为 `1..=3`

  > 1..3: [1, 2] (std::ops::Range), 1..=3: [1, 2, 3] (std::ops::RangeInclusive)

- bool as i32: true -> 1, false -> 0；i32 as bool: 不允许

- #[derive(Debug, PartialEq)]: 让结构体自动实现 Debug trait 和 PartialEq trait，它们的功能是允许结构体实例进行打印和比较。

- “{:?}”：只有实现了 Debug trait，才用有“{:?}”格式化打印的行为。

#### 第三章

- Rust的类型分类
  - Sized Type：可确定大小类型
  - Dynamic Sized Type (DST)：动态大小类型
  - Zero Sized Type (ZST)：零大小类型
  - Bottom Type：底类型， eg: `!`

- 两种转换类型的parse()
  - `let int_x: i32 = x.parse().unwrap();`
  - `x.parse()::<i32>().unwrap()`

  > `::<>` 叫作 turbofish 操作符

- 泛型及单态化：单态化静态分发的好处是性能好，没有运行时开销；缺点是容易造成编译后生成的二进制文件膨胀。（并不影响使用Rust编程）
- <font color=red>孤儿规则： 产生的原因是不是源自可能有默认的 Deref trait，才存在可能影响到其它和T或 ‘a T</font>

- <font color=red>重叠规则：泛型会单例化，而且重叠trait的实现具象化程度不同（相同的话就产生了歧义），那么是不是之间可以在编译时就已经可以为不同情况单例出不同的trait实现选择呢？</font>

#### 第五章

- 借用： 共享不可变，可变不共享

#### 第六章

- fn的函数名称通常以“蛇形命名法（snake_case）”命名



## Day4

### 事件1：《Rust 编程之道》第十章的完整示例代码

- 正则表达式标记：
  - i，匹配时不区分大小写
  - m，多行模式，“^” 和 “$”对应行首和行尾
  - s，允许通配符“.”匹配“\n”
  - U，大写U，交换“x\*”和 “x\*?”的意义
  - u，小写u，允许支持Unicode（默认启用）
  - x，忽略空格并允许行注释（以“#”开头）

- 读写锁控制
  - RwLock: 读写锁，多读单写
  - Metux: 互斥锁，单读写
  
- <font color=red>遇到一个问题：</font>

  > 在实验即将结束的时候，test中`use csv_challenge`都没问题了，main.rs中`use csv_challenge`就报找不到 “csv_challenge”
  >
  > 解决： 删除了Cargo.lock，重新编译

## Day5

### 事件：开始笨办法练习

 

## Day6

### 事件1： 双向链表

- 智能指针
  - Rc<T>： 允许多重拥有，不可变借用，编译时检查
  - Box<T>：单一拥有者，可变或不可变借用，编译时检查
  - RecCell<T>：单一拥有者，可变或不可变借用，运行时检查。可变不可变是对外的，都可以在内部改变。
- <font color=red>一些功能，尤其涉及feature时，需要nightly版本</font>
- 问题：test写了3个，运行cargo test总是0
  - 解决：将test文件下的测试重新移到lib.rs

