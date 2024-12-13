use crate::ast::types::Type;
/**
 * Copyright 2020 Garrit Franke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::lexer::*;
use crate::parser::parse;

#[test]
fn test_parse_empty_function() {
    let raw = "fn main() {}";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_with_return() {
    let raw = "
    fn main() {
        return 1
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_inline_function() {
    let raw = "
    fn greet(name: string) = \"Hello \" + name
    fn main() {
        println(greet(\"World\"))
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
// I don't know how this fails yet. It seems to have something to do with how
// `parse_expression` peeks tokens. It tries to peek a token after the
// expression body but it's empty, so it errors out.
fn test_parse_inline_function_as_last_statement() {
    let raw = "
    fn main() {
        println(greet(\"World\"))
    }
    fn greet(name: string) = \"Hello \" + name
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_redundant_semicolon() {
    let raw = "
    fn main() {
        return 1;
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_err())
}

#[test]
fn test_parse_no_function_context() {
    let raw = "
    let x = 1
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_err())
}

#[test]
fn test_parse_multiple_functions() {
    let raw = "
    fn foo() {
        let x = 2
        return x
    }

    fn bar() {
        let y = 5
        return y
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_variable_declaration() {
    let raw = "
    fn main() {
        let x = 1
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_variable_reassignment() {
    let raw = "
    fn main() {
        let x = 1
        x = x + 1
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_variable_declaration_added() {
    let raw = "
    fn main() {
        let x = 10
        let y = 5
        return x + y
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_with_args() {
    let raw = "
    fn main(foo: int) {
        return foo
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_call() {
    let raw = "
    fn main(foo: int) {
        foo()
    }

    fn foo() {
        foo(2)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_return_function_call() {
    let raw = "
    fn main() {
        return fib(2)
    }

    fn fib() {
        return fib(2)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_call_multiple_arguments() {
    let raw = "
    fn main() {
        fib(1, 2, 3)
    }

    fn fib() {
        return 2
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_nexted_function_call() {
    let raw = "
    fn main() {
        fib(fib(2), 2)
    }

    fn fib(n: int) {
        return 2
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_ops() {
    let raw = "
    fn main() {
        return 2 * 5
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops() {
    let raw = "
    fn main() {
        2 * 5 / 3
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_function_call() {
    let raw = "
    fn main() {
        return 2 * fib(1) / 3
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_function_call_binary_op() {
    let raw = "
    fn main() {
        foo(1) * 2
    }

    fn foo(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse 'foo(1) * 2': {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_complex_function_call_binary_op() {
    let raw = "
    fn main() {
        let x = foo(bar(1, 2)) * 3 + 4
    }

    fn foo(x: int) {
        return x
    }

    fn bar(x: int, y: int) {
        return x + y
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse nested function calls with binary ops: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_function_call_binary_op_in_return() {
    let raw = "
    fn main() {
        return foo(1) * bar(2)
    }

    fn foo(x: int) {
        return x
    }

    fn bar(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse binary op in return statement: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_function_call_binary_op_nested() {
    let raw = "
    fn main() {
        let x = (foo(1) * 2) + (bar(3) * 4)
    }

    fn foo(x: int) {
        return x
    }

    fn bar(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse nested binary operations: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_function_call_binary_op_with_variables() {
    let raw = "
    fn main() {
        let a = 5
        let result = foo(a) * 2
    }

    fn foo(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse binary op with variables: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_function_call_comparison_op() {
    let raw = "
    fn main() {
        if foo(1) > bar(2) {
            return true
        }
        return false
    }

    fn foo(x: int) {
        return x
    }

    fn bar(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse comparison with function calls: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_function_call_binary_op_precedence() {
    let raw = "
    fn main() {
        let result = foo(1) * 2 + bar(3) * 4
    }

    fn foo(x: int) {
        return x
    }

    fn bar(x: int) {
        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(
        tree.is_ok(),
        "Failed to parse operator precedence with function calls: {:?}",
        tree.err()
    );
}

#[test]
fn test_parse_compound_ops_with_strings() {
    let raw = "
    fn main() {
        return 2 * \"Hello\"
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_identifier() {
    let raw = "
    fn main(n: int) {
        return 2 * n
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_with_identifier_first() {
    let raw = "
    fn main(n: int) {
        return n * 2
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_compound_ops_return() {
    let raw = "
    fn main(n: int) {
        return 2 * n
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_conditional() {
    let raw = "
    fn main(n: int) {
        if n {
            return n
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_basic_conditional_with_multiple_statements() {
    let raw = "
    fn main(n: int) {
        if n {
            let x = 2 * n
            return x
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_else_if_branch() {
    let raw = "
    fn main(n: int) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n <= 10 {
            return n
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_multiple_else_if_branch_branches() {
    let raw = "
    fn main(n: int) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n < 10 {
            return n
        } else if n == 10 {
            return n + 1
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_else_branch() {
    let raw = "
    fn main(n: int) {
        if n > 10 {
            let x = 2 * n
            return x
        } else {
            return n
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_parse_conditional_elseif_else_branch() {
    let raw = "
    fn main(n: int) {
        if n > 10 {
            let x = 2 * n
            return x
        } else if n < 10 {
            return n
        } else if n == 10 {
            return n + 1
        } else {
            return n
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_int_array() {
    let raw = "
    fn main() {
        let arr = [1, 2, 3]
        return arr
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_string_array() {
    let raw = "
    fn main(n:int) {
        return [\"Foo\", \"Bar\", \"Baz\"]
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_basic_while_loop() {
    let raw = "
    fn main() {
        let x = 5 * 2
        while x > 0 {
            return x
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_while_loop_boolean_expression() {
    let raw = "
    fn main() {
        let x = 5 * 2
        while true && x {
            return x
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_boolean_arithmetic() {
    let raw = "
    fn main() {
        let x = true && false
        let y = false && true || true
        let z = x && true
        while true && x {
            return x
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_array_access_in_loop() {
    let raw = "
    fn main() {
        let x = [1, 2, 3, 4, 5]

        let i = 0

        while i < 5 {
          println(x[i])
          i += 1
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_array_access_standalone() {
    let raw = "
    fn main() {
        let x = [1, 2, 3, 4, 5]

        x[0]
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_array_access_assignment() {
    let raw = "
    fn main() {
        let arr = [1, 2, 3]
        let x = arr[0]

        return x
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_array_access_in_if() {
    let raw = "
    fn main() {
        if arr[d] > arr[d+1] {
            let swap = arr[d]
            arr[d]   = arr[d+1]
            arr[d+1] = swap
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_uninitialized_variables() {
    let raw = "
    fn main() {
        let x
        let y
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_function_call_math() {
    let raw = "
    fn main(m: int) {
        main(m - 1)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_function_multiple_args() {
    let raw = "
    fn main(m: int, n: int) {
        main(m, n)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_array_position_assignment() {
    let raw = "
    fn main() {
        new_arr[i] = 1
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_typed_declare() {
    let raw = "
    fn main() {
        let x = 5
        let y: int = 1
        let z: bool = false
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok())
}

#[test]
fn test_no_function_args_without_type() {
    let raw = "
    fn main(x) {
        return n
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_err())
}

#[test]
fn test_function_with_return_type() {
    let raw = "
    fn main(x: int): int {
        return n
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
    assert_eq!(tree.unwrap().func[0].ret_type, Some(Type::Int));
}

#[test]
fn test_booleans_in_function_call() {
    let raw = "
    fn main() {
        if n > 2 {
            _printf(true)
        } else {
            _printf(true)
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_late_initializing_variable() {
    let raw = "
    fn main() {
        let x: int
        let y: string
        x = 5
        if x > 2 {
            y = 'test'
        }

        _printf(x)
        _printf(y)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_simple_for_loop() {
    let raw = "
    fn main() {
        let x = [1, 2, 3]

        for i in x {
            _printf(i)
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_nested_for_loop() {
    let raw = "
    fn main() {
        let x = [1, 2, 3]

        for i in x {
            for j in x {
                _printf(j)
            }
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_nested_array() {
    let raw = "
    fn main() {

        let arr = [[11, 12, 13], [21, 22, 23], [31, 32, 33]]
        for i in arr {
            for j in i {
                println(j)
            }
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_simple_nested_expression() {
    let raw = "
    fn main() {
        let x = (1 + 2 * (3 + 2))
        println(x)
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_continue() {
    let raw = "
    fn main() {
        let arr = [1, 2, 3]

        for x in arr {
            if x == 2 {
                continue
            } else {
                println(x)
            }
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_break() {
    let raw = "
    fn main() {
        let arr = [1, 2, 3]

        for x in arr {
            if x == 2 {
                break
            } else {
                println(x)
            }
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_complex_nested_expressions() {
    let raw = "
    fn main() {
        let year = 2020

        if (year % 4 == 0) && (year % 100 != 0) || (year % 400 == 0) {
            println('Leap year')
        } else {
            println('Not a leap year')
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_array_as_argument() {
    let raw = "
    fn main() {
        println([1, 2, 3])
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_struct_initialization() {
    let raw = "
    struct User {
        username: string
        first_name: string
        last_name: string
    }

    fn main() {
        let foo = new User {
            username: 'foobar'
            first_name: 'Foo'
            last_name: 'Bar'
        }
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_arithmetic() {
    // These should pass
    let raw = "fn main() {1*1}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_ok());

    let raw = "fn main() {2+3*4}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_ok());

    let raw = "fn main() {(2+2)*3}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_ok());

    // These should fail
    let raw = "fn main() {(22+)+1}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_err());

    let raw = "fn main() {1++1}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_err());

    let raw = "fn main() {3)+1}";
    let tree = parse(tokenize(raw).unwrap(), Some(raw.to_string()));
    assert!(tree.is_err());
}

#[test]
fn test_array_capacity() {
    let raw = "
    fn main() {
        let arr: int[5]
        arr[0] = 1
        let arr2 = [1, 2, 3]
        let arr3: int[5] = [1, 2, 3, 4, 5]
    }
    ";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()));
    assert!(tree.is_ok());
}

#[test]
fn test_errors_for_struct_decl() {
    let raw = 
    "struct Foo {
        index: int,
        value: int
    }";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()), "".into());
    assert!(tree.is_err());
}


#[test]
fn test_errors_for_struct_literal() {
    let raw = 
    "let foo = new Foo {
        index: 100,
        value: 200
    }";
    let tokens = tokenize(raw).unwrap();
    let tree = parse(tokens, Some(raw.to_string()), "".into());
    assert!(tree.is_err());
}

