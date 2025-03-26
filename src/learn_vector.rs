#![allow(dead_code)]

pub fn sort_vector() {
    let mut vec_1 = vec![1, 5, 10, 7, 2, 3, 4, 6, 8, 9];
    vec_1.sort();
    assert_eq!(vec_1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "sorted vector");
}

pub fn sort_vector_float() {
    let mut vec_1 = vec![1.0, 5.0, 10.0, 7.0, 2.0, 3.0, 4.0, 6.0, 8.0, 9.0];
    vec_1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(
        vec_1,
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        "sorted vector"
    );
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

pub fn sort_struct_vector() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25)
        ]
    );

    println!("{:?}", people);

    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1)
        ]
    );

    println!("{:?}", people);
}

pub fn vector_iter() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // `vec!` 宏允许在创建向量的同时初始化值
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    xs.push(4);
    println!("Vector after push: {:?}", xs);

    xs.pop();   
    println!("Vector after pop: {:?}", xs);

    // 迭代一个vector很容易
    for x in xs.iter() {
        println!("> {}", x);
    }

    // 可以在迭代的同时，使用独立的变量i记录迭代次数
    for (i, x) in xs.iter().enumerate() {
        println!("> {}: {}", i, x);
    }

    // 多亏了`iter_mut`，可以得到一个可变迭代器
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Vector after mutate: {:?}", xs);  
    
    
    
}