use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("at least one arg is required")
    }
    parse(args.get(1).unwrap());
}

fn parse(input: &str) -> u32 {
    let mut code = input;
    let mut code = & mut code;

    while code.len() > 0 {
        let result = eval( &mut  code);
        return result;
    }
    return 0;
}

fn eval(code:  & mut &str) -> u32 {
    println!("{:?}", &code[0..1]);
    let chars = code.chars();
    let mut pos: usize = 0;
    let mut num: u32 = 0;
    for (i, c) in chars.enumerate() {
        if let Some(digit) = c.to_digit(10) {
            pos = i;
            num = digit + num * 10;
            println!("p:{}", i);
            println!("n:{}", num);
            
        }
        if c.is_whitespace() {
            println!("number?: {:?}", &code[..pos+1]);
            *code = &code[pos..];
            return 0
        }
    }
    println!("number?: {:?}", &code[..pos+1]);
    println!("number: {:?}", num);
    *code = &code[pos..];
    num
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use parse;
        assert_eq!(parse("1234"), 1234);
    }
}