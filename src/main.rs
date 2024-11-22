use std::fs::File;
use std::io::Write;
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Print,
    LParen,
    RParen,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.next_char() {
            Some(c) if c.is_alphabetic() => {
                let mut identifier = c.to_string();
                while let Some(next) = self.peek_char() {
                    if next.is_alphanumeric() {
                        identifier.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                
                if identifier == "print" {
                    return Token::Print;
                }

                Token::Identifier(identifier)
            }
            Some(c) if c.is_numeric() => {
                let mut number = c.to_string();
                while let Some(next) = self.peek_char() {
                    if next.is_numeric() {
                        number.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse::<f64>().unwrap())
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('=') => Token::Assign,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('*') => Token::Multiply,
            Some('/') => Token::Divide,
            None => Token::EOF,
            Some(c) => panic!("Unexpected character in input: '{}'", c),
        }
    }
}

#[derive(Debug)]
enum ASTNode {
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    BinaryOp {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Number(f64),
    Variable(String),
    Print {
        variable: String,
    },
}

impl ASTNode {
    fn collect_variables(node: &ASTNode, vars: &mut Vec<String>) {
        match node {
            ASTNode::Assignment { variable, .. } => {
                if !vars.contains(variable) {
                    vars.push(variable.clone());
                }
            }
            ASTNode::BinaryOp { left, right, .. } => {
                ASTNode::collect_variables(left, vars);
                ASTNode::collect_variables(right, vars);
            }
            _ => {}
        }
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.current_token = self.lexer.next_token();
        } else {
            panic!(
                "Unexpected token: {:?}, expected: {:?}",
                self.current_token, token
            );
        }
    }

    fn parse_term(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Number(value) => {
                self.eat(Token::Number(value));
                ASTNode::Number(value)
            }
            Token::Identifier(name) => {
                self.eat(Token::Identifier(name.clone()));
                ASTNode::Variable(name)
            }
            _ => panic!("Unexpected token in term: {:?}", self.current_token),
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        let mut left = self.parse_term();

        while matches!(self.current_token, Token::Plus | Token::Minus | Token::Multiply | Token::Divide) {
            let operator = match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    "+"
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    "-"
                }
                Token::Multiply => {
                    self.eat(Token::Multiply);
                    "*"
                }
                Token::Divide => {
                    self.eat(Token::Divide);
                    "/"
                }
                _ => unreachable!(),
            }
            .to_string();

            let right = self.parse_term();
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_assignment(&mut self) -> ASTNode {
        if let Token::Identifier(name) = self.current_token.clone() {
            self.eat(Token::Identifier(name.clone()));
            self.eat(Token::Assign);
            let value = self.parse_expression();
            ASTNode::Assignment {
                variable: name,
                value: Box::new(value),
            }
        } else {
            panic!("Expected an identifier for assignment");
        }
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            if self.current_token == Token::Print {
                self.eat(Token::Print);
                self.eat(Token::LParen);
                if let Token::Identifier(name) = self.current_token.clone() {
                    self.eat(Token::Identifier(name.clone()));
                    self.eat(Token::RParen);
                    nodes.push(ASTNode::Print { variable: name });
                } else {
                    panic!("Expected an identifier inside 'print(...)'");
                }
            } else {
                nodes.push(self.parse_assignment());
            }
        }
        nodes
    }
}

struct CodeGenerator {
    output: File,
}

impl CodeGenerator {
    fn new(output_path: &str) -> Self {
        let file = File::create(output_path).expect("Unable to create file");
        CodeGenerator { output: file }
    }

    fn emit(&mut self, instruction: &str) {
        writeln!(self.output, "{}", instruction).expect("Unable to write to file");
    }

    fn emit_header(&mut self, variables: &[String]) {
        self.emit("section .bss");
        for var in variables {
            self.emit(&format!("{} resq 1", var));
        }
        self.emit("buffer resb 20");

        self.emit("section .data");
        self.emit("newline db 0xA, 0");
        self.emit("error_message db \"Error: Division by zero\", 0xA, 0");
        self.emit("error_len equ $ - error_message");

        self.emit("section .text");
        self.emit("global _start");
        self.emit("_start:");
    }

    fn emit_footer(&mut self) {
        self.emit("    mov rax, 60");
        self.emit("    xor rdi, rdi");
        self.emit("    syscall");

        self.emit("division_by_zero:");
        self.emit("    mov rax, 1");
        self.emit("    mov rdi, 1");
        self.emit("    mov rsi, error_message");
        self.emit("    mov rdx, error_len");
        self.emit("    syscall");
        self.emit("    mov rax, 60");
        self.emit("    xor rdi, rdi");
        self.emit("    syscall");
    }

    fn generate(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Assignment { variable, value } => {
                self.generate(value);
                self.emit(&format!("    mov [{}], rax", variable));
            }
            ASTNode::BinaryOp { left, operator, right } => {
                self.generate(right);
                self.emit("    push rax");
                self.generate(left);
                self.emit("    pop rbx");
                match operator.as_str() {
                    "+" => self.emit("    add rax, rbx"),
                    "-" => self.emit("    sub rax, rbx"),
                    "*" => self.emit("    imul rax, rbx"),
                    "/" => {
                        self.emit("    cmp rbx, 0");
                        self.emit("    je division_by_zero");
                        self.emit("    xor rdx, rdx");
                        self.emit("    div rbx");
                    }
                    _ => panic!("Unsupported operator"),
                }
            }
            ASTNode::Number(value) => {
                self.emit(&format!("    mov rax, {}", *value as i64));
            }
            ASTNode::Variable(name) => {
                self.emit(&format!("    mov rax, [{}]", name));
            }
            ASTNode::Print { variable } => {
                self.emit(&format!("    mov rax, [{}]", variable));
                self.emit("    mov rcx, buffer");
                self.emit("    call int_to_string");
                self.emit("    mov rdx, buffer");
                self.emit("    add rdx, 20");
                self.emit("    sub rdx, rcx");
                self.emit("    mov rsi, rcx");
                self.emit("    mov rax, 1");
                self.emit("    mov rdi, 1");
                self.emit("    syscall");

                self.emit("    mov rsi, newline");
                self.emit("    mov rdx, 1");
                self.emit("    mov rax, 1");
                self.emit("    mov rdi, 1");
                self.emit("    syscall");
            }
        }
    }
}

fn main() {
    let source_path = "example.vp";
    let source_code = fs::read_to_string(source_path).unwrap();

    let lexer = Lexer::new(&source_code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    let mut variables = Vec::new();
    for node in &ast {
        ASTNode::collect_variables(node, &mut variables);
    }

    let mut codegen = CodeGenerator::new("output.asm");
    codegen.emit_header(&variables);
    for node in ast {
        codegen.generate(&node);
    }
    codegen.emit_footer();

    println!("Assembly code generated in output.asm");
}
