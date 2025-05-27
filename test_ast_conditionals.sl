fn main() {
    // Test if statement without else
    let x: int = 5;
    if x > 3 {
        print_value("greater");
    }
    
    // Test if statement with else
    if x > 10 {
        print_value("very large");
    } else {
        print_value("not so large");
    }
    
    // Test conditional expression
    let result: string = if x > 0 { "positive" } else { "negative" };
    print_value(result);
}
