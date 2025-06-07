# Grammar

Grammar of the slang language in the extended backus naur format:

```ebnf
/* Program structure */
program = { statement ";" | comment } ;

/* Comments */
comment = single_line_comment | multi_line_comment ;

single_line_comment = "//", { character - "\n" }, [ "\n" ] ;

multi_line_comment = "/*", { character | multi_line_comment }, "*/" ;

/* Statements */
statement = let_statement
          | expression_statement
          | type_definition_statement
          | function_declaration_statement
          | return_statement
          | if_statement ;

let_statement = "let", ["mut"], identifier, [ ":", type ], "=", expression, ";" ;

expression_statement = expression, ";" ;

type_definition_statement = "struct", identifier, "{", { field_definition, [ "," ] }, "}", ";" ;

field_definition = identifier, ":", type ;

function_declaration_statement = "fn", identifier, "(", [ parameter_list ], ")", [ "->", type ], block_expression ;

parameter_list = parameter, { ",", parameter } ;

parameter = identifier, ":", type ;

return_statement = "return", [ expression ], ";" ;

if_statement = "if", expression, block_expression, [ "else", ( if_statement | block_expression ) ] ;

/* Expressions */
expression = logical_or ;

logical_or = logical_and, { "||", logical_and } ;

logical_and = equality, { "&&", equality } ;

equality = relational, { ( "==" | "!=" ), relational } ;

relational = term, { ( ">" | "<" | ">=" | "<=" ), term } ;

term = factor, { ( "+" | "-" ), factor } ;

factor = unary, { ( "*" | "/" ), unary } ;

unary = [ "-" | "!" ], primary ;

primary = literal
        | identifier
        | call_expression
        | if_expression
        | block_expression
        | "(", expression, ")" ;

call_expression = identifier, "(", [ argument_list ], ")" ;

if_expression = "if", expression, "{", expression, "}", "else", "{", expression, "}" ;

block_expression = "{", { statement }, [ expression ], "}" ;

argument_list = expression, { ",", expression } ;

/* Literals */
literal = integer_literal
        | float_literal
        | string_literal
        | boolean_literal ;

integer_literal = digit, { digit }, [ integer_type_suffix ] ;

integer_type_suffix = "i32" | "i64" | "u32" | "u64" ;

float_literal = digit, { digit }, ".", { digit }, [ float_type_suffix ] ;

float_type_suffix = "f32" | "f64" ;

string_literal = '"', { character - '"' }, '"' ;

boolean_literal = "true" | "false" ;

/* Types */
type = "i32" | "i64" | "u32" | "u64" | "f32" | "f64" | "string" | "bool" | identifier ;

/* Lexical elements */
identifier = letter, { letter | digit | "_" } ;

letter = "A" | "B" | ... | "Z" | "a" | "b" | ... | "z" ;

digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

character = ? any ASCII character ? ;
```
