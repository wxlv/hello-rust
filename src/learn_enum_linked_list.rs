#![allow(dead_code)]
use List::*;

// 定义一个包含所有可能的列表节点的枚举
enum List {
    // Cons: 元组结构体，包含一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil: 末节点，表明链表结束
    Nil,
}

// 为 List 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型的一个变体(`Nil`的完整名称是`List::Nil`)
        List::Nil
    }
    // 处理一个 List, 在其头部插入一个新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型的变体
        List::Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // `self` 必须匹配，因为这个方法的行为取决于 `self` 的取值类型
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的`T`类型要好过匹配引用`&T`
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；因此使用一个对tail的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为0的空列表
            Nil => 0,
        }
    }

    // 返回 List 的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回一个堆分配的字符串而不是打印结果到控制台
                format!("{}, {}", head, tail.stringify())
            }
            Nil => "Nil".to_string(),
        }
    }
}

/// Demonstrates the usage of an enum-based linked list.
///
/// This function creates a new linked list, adds some elements to it, and then
/// prints the length and string representation of the final list.
pub fn enum_linked_list() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些数据
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
