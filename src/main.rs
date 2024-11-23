use std::fs;
use std::fs::File;
use std::io::Write;

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
    If,
    Else,
    LParen,
    RParen,
    LBrace,
    RBrace,
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

                match identifier.as_str() {
                    "print" => Token::Print,
                    "if" => Token::If,
                    "else" => Token::Else,
                    _ => Token::Identifier(identifier),
                }
            }
            Some(c) if c.is_numeric() => {
                let mut number = c.to_string();
                while let Some(next) = self.peek_char() {
                    if next.is_numeric() || next == '.' {
                        number.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number.parse::<f64>().unwrap())
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Multiply,
            Some('/') => Token::Divide,
            Some('=') => Token::Assign,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
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
        expression: Box<ASTNode>,
    },
    If {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Vec<ASTNode>,
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
            ASTNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                ASTNode::collect_variables(condition, vars);
                for stmt in then_branch.iter().chain(else_branch.iter()) {
                    ASTNode::collect_variables(stmt, vars);
                }
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
        Parser {
            lexer,
            current_token,
        }
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

        while matches!(
            self.current_token,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide
        ) {
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

    fn parse_comparison(&mut self) -> ASTNode {
        let left = self.parse_expression();

        if let Token::Assign = self.current_token {
            self.eat(Token::Assign);
            if let Token::Assign = self.current_token {
                self.eat(Token::Assign);
                let right = self.parse_expression();
                return ASTNode::BinaryOp {
                    left: Box::new(left),
                    operator: "==".to_string(),
                    right: Box::new(right),
                };
            } else {
                panic!(
                    "Unexpected token: {:?}. Expected '=' for comparison.",
                    self.current_token
                );
            }
        }

        left
    }
    fn parse_if(&mut self) -> ASTNode {
        self.eat(Token::If);
        self.eat(Token::LParen);
        let condition = self.parse_comparison(); // handle comparisons here
        self.eat(Token::RParen);
        self.eat(Token::LBrace);
        let then_branch = self.parse_block();
        self.eat(Token::RBrace);

        let else_branch = if self.current_token == Token::Else {
            self.eat(Token::Else);
            self.eat(Token::LBrace);
            let branch = self.parse_block();
            self.eat(Token::RBrace);
            branch
        } else {
            vec![]
        };

        ASTNode::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }
    }
    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }
        statements
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token {
            Token::If => self.parse_if(),
            Token::Print => {
                self.eat(Token::Print);
                self.eat(Token::LParen);
                let expr = self.parse_comparison(); // Updated to handle comparisons
                self.eat(Token::RParen);
                ASTNode::Print {
                    expression: Box::new(expr),
                }
            }
            Token::Identifier(_) => self.parse_assignment(),
            _ => panic!(
                "Unexpected token: {:?}. Expected a statement.",
                self.current_token
            ),
        }
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        nodes
    }
}

struct CodeGenerator {
    output: File,
    label_counter: usize,
}

impl CodeGenerator {
    fn new(output_path: &str) -> Self {
        let file = File::create(output_path).expect("Unable to create file");
        CodeGenerator {
            output: file,
            label_counter: 0,
        }
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

        self.emit("section .text");
        self.emit("global _start");
        self.emit("_start:");
    }

    fn emit_footer(&mut self) {
        self.emit("    mov rax, 60       ; syscall: exit");
        self.emit("    xor rdi, rdi      ; return code: 0");
        self.emit("    syscall");

        self.emit("; Subroutine to convert an integer in RAX to a string in the buffer");
        self.emit("int_to_string:");
        self.emit("    xor rdx, rdx              ; Clear rdx (remainder)");
        self.emit("    mov rbx, 10               ; Divisor for decimal system");
        self.emit("    add rcx, 20               ; Move pointer to the end of the buffer");
        self.emit("    dec rcx                   ; Reserve space for the last character");
        self.emit(".convert_loop:");
        self.emit("    xor rdx, rdx              ; Clear rdx before division");
        self.emit("    div rbx                   ; Divide rax by 10, remainder in rdx");
        self.emit("    add dl, '0'               ; Convert remainder to ASCII");
        self.emit("    mov [rcx], dl             ; Store the ASCII character in the buffer");
        self.emit("    dec rcx                   ; Move to the previous position in the buffer");
        self.emit("    test rax, rax             ; Check if quotient is 0");
        self.emit("    jnz .convert_loop         ; Repeat if not 0");
        self.emit("    inc rcx                   ; Adjust pointer to the start of the string");
        self.emit("    ret");
    }

    fn new_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!("{}_{}", prefix, self.label_counter)
    }

    fn generate(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Assignment { variable, value } => {
                self.generate(value);
                self.emit(&format!("    mov [{}], rax", variable));
            }
            ASTNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                self.generate(right);
                self.emit("    push rax");
                self.generate(left);
                self.emit("    pop rbx");
                match operator.as_str() {
                    "+" => self.emit("    add rax, rbx"),
                    "-" => self.emit("    sub rax, rbx"),
                    "*" => self.emit("    imul rax, rbx"),
                    "/" => {
                        self.emit("    xor rdx, rdx");
                        self.emit("    div rbx");
                    }
                    "==" => {
                        self.emit("    cmp rax, rbx");
                        self.emit("    sete al"); // at to 1 if equal
                        self.emit("    movzx rax, al"); // zero etend al to rax
                    }
                    _ => panic!("Unsupported operator: {}", operator),
                }
            }
            ASTNode::Number(value) => {
                self.emit(&format!("    mov rax, {}", *value as i64));
            }
            ASTNode::Variable(name) => {
                self.emit(&format!("    mov rax, [{}]", name));
            }
            ASTNode::Print { expression } => {
                self.generate(expression);
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
            ASTNode::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.generate(condition);
                self.emit("    cmp rax, 0");
                let else_label = self.new_label("else");
                let end_label = self.new_label("end_if");
                self.emit(&format!("    je {}", else_label));
                for stmt in then_branch {
                    self.generate(stmt);
                }
                self.emit(&format!("    jmp {}", end_label));
                self.emit(&format!("{}:", else_label));
                for stmt in else_branch {
                    self.generate(stmt);
                }
                self.emit(&format!("{}:", end_label));
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
