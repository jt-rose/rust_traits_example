fn main() {
    // instantiate values
    let num = Num { n: 32 };
    let word = Word {
        w: "hello".to_string(),
    };

    // call double_me trait
    let res1 = num.double_me();
    let res2 = word.double_me();

    // print results
    println!("results are {} and {}", res1, res2);
}

// declare public trait with generic type
pub trait Double<T> {
    fn double_me(&self) -> T;
}

// set up structs and implementations
pub struct Num {
    n: i32,
}

impl Double<i32> for Num {
    fn double_me(&self) -> i32 {
        let res = &self.n + &self.n;
        res
    }
}

pub struct Word {
    w: String,
}

impl Double<String> for Word {
    fn double_me(&self) -> String {
        let mut copy = self.w.clone();
        copy.push_str(&self.w);
        copy
    }
}
