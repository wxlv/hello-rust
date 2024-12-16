use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{0}:{1}", count, v)?;
        }

        write!(f, "]")
    }
}

pub fn list_display() {
    let v = List(vec![19, 122, 3313, 0120, 339]);
    println!("{}", v);
}
