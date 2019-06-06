extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

use hello_macro::HelloMacro;

#[derive(HelloMacro)]
struct Test {
    memeber_one: i32,
}

impl Test {
    fn new() -> Test {
        Test { memeber_one: 0 }
    }

    fn nothing(&self) {
        println!("{}", self.memeber_one);
    }
}

fn main() {
    let mytest: Test = Test::new();
    mytest.nothing();
    Test::hello_macro();
}
