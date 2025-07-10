# Viper Programming Language

**WIP** - It's like Python, but faster.

Viper is a simple, Python-like programming language that compiles to x86-64 assembly. It features a clean syntax and efficient compilation to native code.

## Features

### Core Language Features
- **Variables and Assignment**: `x = 5`
- **Arithmetic Operations**: `+`, `-`, `*`, `/`
- **Print Statements**: `print(expression)`

### Control Flow
- **Conditional Statements**: `if (condition) { ... } else { ... }`
- **While Loops**: `while (condition) { ... }`
- **For Loops**: `for (variable in range(n)) { ... }`
- **Loop Control**: `break`, `continue` (planned)

### Comparison Operators
- **Equality**: `==`, `!=`
- **Relational**: `<`, `>`, `<=`, `>=`

### Built-in Functions
- **range(n)**: Generate numbers from 0 to n-1 (for use in for loops)
- **print(expr)**: Output the value of an expression

## Syntax Examples

### Basic Operations
```python
# Variables and arithmetic
x = 10
y = 5
sum = x + y
print(sum)  # Outputs: 15
```

### Conditional Statements
```python
if (x > y) {
    print(1)  # x is greater
} else {
    print(0)  # y is greater or equal
}
```

### Loops
```python
# While loop
counter = 0
while (counter < 5) {
    print(counter)
    counter = counter + 1
}

# For loop
for (i in range(5)) {
    print(i)  # Prints 0, 1, 2, 3, 4
}
```

## Calculator Example

The `calculator.vp` file demonstrates various Viper features:

```python
# Factorial calculation
fact = 1
for (i in range(6)) {
    if (i > 1) {
        fact = fact * i
    }
}
print(fact)  # Outputs: 120 (5!)

# Power calculation
base = 2
exponent = 3
power = 1
for (i in range(exponent)) {
    power = power * base
}
print(power)  # Outputs: 8 (2^3)

# Sum of numbers 1 to 10
total = 0
for (i in range(11)) {
    if (i > 0) {
        total = total + i
    }
}
print(total)  # Outputs: 55
```

## Building and Running

1. **Build the compiler**:
   ```bash
   cargo build --release
   ```

2. **Compile a Viper program**:
   ```bash
   ./target/release/viper example.vp
   ```

3. **Assemble and link** (Linux x86-64):
   ```bash
   nasm -f elf64 output.asm -o output.o
   ld output.o -o program
   ./program
   ```

## Language Grammar

```
program         → statement*
statement       → assignment | print_stmt | if_stmt | while_stmt | for_stmt | break_stmt | continue_stmt
assignment      → IDENTIFIER "=" expression
print_stmt      → "print" "(" expression ")"
if_stmt         → "if" "(" comparison ")" "{" statement* "}" ("else" "{" statement* "}")?
while_stmt      → "while" "(" comparison ")" "{" statement* "}"
for_stmt        → "for" "(" IDENTIFIER "in" expression ")" "{" statement* "}"
break_stmt      → "break"
continue_stmt   → "continue"
comparison      → expression (("==" | "!=" | "<" | ">" | "<=" | ">=") expression)?
expression      → term (("+"|"-"|"*"|"/") term)*
term            → NUMBER | IDENTIFIER | "(" expression ")" | function_call
function_call   → IDENTIFIER "(" expression ")"
```

## TODO

### Language Features
- [ ] Implement break and continue statements properly
- [ ] Add support for floating-point arithmetic
- [ ] Function definitions and calls
- [ ] Arrays and string support
- [ ] More built-in functions

### Compiler Improvements
- [ ] Implement arbitrary precision arithmetic
- [ ] Multi-Platform Support - Add support for macOS (Mach-O)
- [ ] Cross-compilation support
- [ ] Better error reporting and debugging
- [ ] Keep compiler frontend decoupled from backend-specific details
- [ ] Optimize generated assembly code

### Development Tools
- [ ] Syntax highlighting
- [ ] REPL (Read-Eval-Print Loop)
- [ ] Standard library
- [ ] Package manager

## Contributing

Viper is a work in progress. Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is open source and available under the MIT License.

