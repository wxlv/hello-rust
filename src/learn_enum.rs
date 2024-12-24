// 该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    Click(i64, i64),
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click(x, y) => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

pub fn enum_test(){
    let presses = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click(12, 34);
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(presses);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}