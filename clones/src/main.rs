use std::env;

// must call with two command-line parameters
fn main() {
    let some_vec: Vec<String> = env::args().collect();
    let s = S::new(&some_vec);

    println!(
        "got struct S with foo {} and bar {}",
        s.foo_string, s.bar_string
    );
}

// do not change this struct definition
struct S {
    foo_string: String,
    bar_string: String,
}

// you will need to change the impl
impl S {
    fn new(args: &[String]) -> S {
        let foo_string = args[1].clone();
        let bar_string = args[2].clone();

        S {
            foo_string,
            bar_string,
        }
    }
}
