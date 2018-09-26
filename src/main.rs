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
    skip_whitespace(code);
    let mut chars = code.chars();
    let mut pos: usize = 0;
    let mut num: u32 = 0;
    let first_char = chars.next().unwrap();
    if first_char.is_digit(10) {
        num = first_char.to_digit(10).unwrap();
        pos += 1;
        for (i, c) in chars.enumerate() {
            if let Some(digit) = c.to_digit(10) {
                pos = i;
                num = digit + num * 10;
                println!("p:{}", i);
                println!("n:{}", num);
            } else {
                pos = i + 1;
                *code = &code[pos..];
                return num;
            }
        }
        *code = &code[pos..];
        return num
    } else {
        match first_char {
            '+' => {
                *code = &code[pos + 1..];
                num = eval(code) + eval(code);
                num
                },
            _ => panic!("illegal character"),
        }
    }

}

fn skip_whitespace(code: &mut &str) {
    let chars = code.chars();
    let mut pos: usize = 0;
    for (i, c) in chars.enumerate() {
        if c.is_whitespace() {
            pos = i;
        } else {
            pos = i;
            *code = &code[pos..];
            return
        }
    }
    *code = &code[pos..];
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use parse;
        assert_eq!(parse("1234"), 1234);
        assert_eq!(parse("+ 1 2"), 3);
        assert_eq!(parse("+ + 1 2 3"), 6);
    }
}