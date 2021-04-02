use std::env;
use std::iter::{Iterator, Peekable};
// use anyhow::{anyhow, Result};

// NO ERROR HUNDLING to simplify the code.
fn compile(exp: &str) {
    let mut iter = exp.chars().peekable();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("    mov rax, {}", strtou(&mut iter));

    loop {
        match iter.next() {
            Some(val) => {
                match val {
                    '+' => {
                        // add
                        println!("    add rax, {}", strtou(&mut iter));
                    },
                    '-' => {
                        // sub
                        println!("    sub rax, {}", strtou(&mut iter));
                    },
                    _ => {
                        println!("Unexpected operator: use +, -");
                        break;
                    }
                }
            },
            None => {
                break;
            }
        }
    }
    println!("    ret");
}

fn strtou<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> usize {
    let mut result: usize = 0;
    loop {
        match iter.peek() {
            Some(c) => match c.to_digit(10) {
                Some(i) => result = result * 10 + i as usize,
                None => break,
            },
            None => break,
        }
        iter.next();
    }
    result
}


// -----------------------------------------------
fn main() {
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid number of command line arguments");
    }

    // compile
    compile(&args[1]);
}


// ----------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile() {
        compile("1+2");
        compile("1+10");
        compile("1+9999");
        compile("5+20-4");
    }

    #[test]
    #[should_panic]
    fn test_compile_panic_0() {
        compile("?100");
    }
}
