use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("at least one arg is required")
    }
    let p = args.get(1).unwrap();

    println!("{:?}", p.len());
    let mut value :i32 = 0;
    let mut index :usize = 0;
    while index != p.len() - 1 {
        let value_index = eval(p.to_string(), index);
        value = value_index.0;
        index = value_index.1;
    }
    println!("value: {}", value);
    println!("index: {}", index);
}

fn eval(_p: String, i:  usize) -> (i32,  usize) {
//    println!("debug: {}", *i + 1 );
    (0, i + 1)
}