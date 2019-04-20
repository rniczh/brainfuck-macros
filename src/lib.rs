extern crate proc_macro;

use crate::proc_macro::TokenStream;

#[proc_macro]
pub fn brainfuck(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let _cap = 10000;           // capacity of vector

    let tokens = input
        .trim_matches(char::is_whitespace)
        .chars()
        .enumerate();

    let mut res = vec![];

    for (index, token) in tokens {
        match token {
            '>' => res.push(format!("_p = _p.offset(1);")),
            '<' => res.push(format!("_p = _p.offset(-1);")),
            '+' => res.push(format!("*_p += 1;")),
            '-' => res.push(format!("*_p -= 1;")),
            '.' => res.push(format!("print!(\"{{}}\", *_p as char);")),
            ',' => res.push(format!("*_p = read_char() as u8;")),
            '[' => res.push(format!("while *_p != 0 {{")),
            ']' => res.push(format!("}}")),
            w if w.is_whitespace() => {},
            _ => panic!("Not correct syntax: {} at index {}", token, index),
        }
    }

    let code = format!(
        "{{
        fn read_char() -> char {{
            use std::io::Read;
            let input: char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as char)
                .expect(\"Not supported char\");
            input
        }}
        //use libc::getchar; // it will need user to use libc...
        let mut _vec = ::std::vec::from_elem(0u8,{});
        let mut _p = _vec.as_mut_ptr();
        let mut _i = 0usize;
        unsafe {{
            {}
        }}
        }}
        ", _cap, res.iter().map(|s| format!("{}", s)).collect::<String>());

    code.parse().unwrap()
}
