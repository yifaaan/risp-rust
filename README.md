# What is this?
This is a Lisp interpreter, written in Rust.

Supported by the interpreter:
- Data types
    - integer
    - boolean
- Statements
    - variable definition and assignment
    - if-else
    - function definition using lambdas
    - function calls
- Keywords
    - define
    - if-else
    -lambda

# Structure

- Lexer
- Parser
- Evaluator
- REPL

# Examples

```Lisp
    (
        (define factorial (lambda (n) (if (< n 1) 1 (* n (factorial (- n 1))))))
        (factorial 5)
    )
```

```Lisp
    (
        (define pix 314)
        (define r 10)
        (define sqr (lambda (r) (* r r)))
        (define area (lambda (r) (* pix (sqr r))))
        (area r)
    )
```