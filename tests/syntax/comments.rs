use crate::test_utils::execute_program_and_assert;

#[test]
fn test_single_line_comments() {
    let program = r#"
        let x = 5; // This is a single-line comment
        // This entire line is a comment
        let y = 10;
        
        // Comment before a calculation
        let z = x + y; // Comment after a calculation
        
        print_value(z);
    "#;
    
    execute_program_and_assert(program, "15");
}

#[test]
fn test_multi_line_comments() {
    let program = r#"
        let x = 5;
        /* This is a 
           multi-line comment
           that spans several lines */
        let y = 10;
        
        /* Comment before
           a calculation */
        let z = x + y;
        
        print_value(z);
    "#;
    
    execute_program_and_assert(program, "15");
}

#[test]
fn test_nested_multi_line_comments() {
    let program = r#"
        let x = 5;
        /* Outer comment
           /* Nested comment */
           Still in outer comment */
        let y = 10;
        
        let z = x + y;
        
        print_value(z);
    "#;
    
    execute_program_and_assert(program, "15");
}

#[test]
fn test_comments_in_expressions() {
    let program = r#"
        let x = 5 /* inline comment */ + 10;
        let y = 20 // End of line comment
            + 30; // This works because semicolons terminate statements
        
        let z = x /* comment between 
                    operands */ + y;
        
        print_value(z);
    "#;
    
    execute_program_and_assert(program, "65");
}

#[test]
fn test_comments_in_complex_code() {
    let program = r#"
        // Function definition
        fn add(a: i32, b: i32) -> i32 {
            // Return the sum
            return a + b;
        }
        
        /* Main program logic */
        let x: i32 = 10; // Initialize x
        let y: i32 = 20; // Initialize y
        
        // Call the function
        let z = add(
            x, /* first argument */
            y  /* second argument */
        );
        
        /* Output
           the result */
        print_value(z);
    "#;
    
    execute_program_and_assert(program, "30");
}