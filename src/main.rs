use std::iter::{Iterator, Peekable};
use std::str::Chars;
use anyhow::{anyhow, Result};

/*
Grammer

expr = mul ("+" mul | "-" mul)*
mul = primary ("*" primary | "/" primary)*
primary = num | "(" expr ")"
*/

#[derive(Debug)]
enum NodeKind {
    Op(char),
    Num(usize),
}

type Link = Option<Box<Node>>;

struct Node {
    kind: NodeKind,
    lhs: Link,
    rhs: Link,
}

impl Node {
    fn new(kind: NodeKind, lhs: Link, rhs: Link) -> Self {
        Self {kind, lhs, rhs}
    }

    fn link(node: Node) -> Link {
        Some(Box::new(node))
    }
}

// Input makes node tree from string
struct Input<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Input<'a> {
    fn new(input: &'a str) -> Self {
        let iter = input.chars().peekable();
        Self {input: iter}
    }

    fn tokenize(&mut self) -> Node {
        let head_node = self.expr();
        head_node
    }
    
    // expr = mul ('+' mul | '-' mul)*
    fn expr(&mut self) -> Node {
        // println!("expr");
        let mut node = self.mul();

        loop {
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '+' => {
                            // println!("operator: {}", c);
                            self.input.next();
                            node = Node::new(NodeKind::Op('+'), Node::link(node), Node::link(self.mul()));
                        },
                        '-' => {
                            // println!("operator: {}", c);
                            self.input.next();
                            node = Node::new(NodeKind::Op('-'), Node::link(node), Node::link(self.mul()));
                        },

                        ' ' => {
                            self.input.next();
                        },

                        ')' => {
                            return node;
                        }

                        _ => {
                            panic!("Invalid Operator: {}", c);
                        }
                    }
                },
                None => {
                    return node;
                }
            }
        }

    }
    
    // mul = primary ('*' primary | '/' primary)*
    fn mul(&mut self) -> Node {
        // println!("mul");
        let mut node = self.primary();

        loop {
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '*' => {
                            // println!("operator: {}", c);
                            self.input.next();
                            node = Node::new(NodeKind::Op('*'), Node::link(node), Node::link(self.primary()));
                            
                        },
                        '/' => {
                            // println!("operator: {}", c);
                            self.input.next();
                            node = Node::new(NodeKind::Op('/'), Node::link(node), Node::link(self.primary()));
                        },

                        ' ' => {
                            self.input.next();
                        }

                        _ => {
                            return node;
                        }
                    }
                },
                None => {
                    return node;
                }
            }
        }
    }
    
    // primary = num | '(' expr ')'
    fn primary(&mut self) -> Node {
        // println!("primary");
        loop {
            match self.input.peek() {
                Some(&c) => {
                    match c {
                        '0'..='9' => {
                            let num = strtou(&mut self.input);
                            // println!("digit: {}", num);
                            let node = Node::new(NodeKind::Num(num), None, None);
                            return node;
                        },
                        '(' => {
                            self.input.next();
                            let node = self.expr();
                            if *self.input.peek().unwrap() == ')' {
                                self.input.next();
                            } else {
                                panic!("')' not found!");
                            }
                            return node;
                        },

                        ' ' => {
                            self.input.next();
                        }
                        _ => {
                            panic!("Invalid value: {}", c);
                        }
                    }
                },
                None => {
                    panic!("Expected some value!");
                }
            }
        }
    }
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

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_node() {
        let node = test_tokenize("1*(2+3)");
        print_node(&node);
        println!("-------------");
        
        test_tokenize("1 + 20+ 4");
        test_tokenize(" 9- 6 * 10");
        
        let node = test_tokenize("1-10/100 +1000 * 10000");
        print_node(&node);
        println!("-------------");
        
        let node = test_tokenize("((2-20)*200 + 2000)*(21 - 201)");
        print_node(&node);
        println!("-------------");

    }

    fn print_node(node: &Node) {
        println!("{:?}", node.kind);
        if let Some(n) = &node.lhs {
            print_node(n);
        }
        
        if let Some(n) = &node.rhs {
            print_node(n);
        }
    }

    fn test_tokenize(s: &str) -> Node {
        let mut input = Input::new(s);
        let head = input.tokenize();
        head
    }
}





/*
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
*/
