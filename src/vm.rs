use crate::bytecode::{Chunk, Function, NativeFunction, OpCode, Value};
use std::collections::HashMap;

// Call frame to track function calls
struct CallFrame {
    function: Function,
    return_address: usize,
    stack_offset: usize,
    locals: HashMap<String, Value>, // Track local variables for this frame
}

pub struct VM {
    ip: usize,  // Instruction pointer
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    frames: Vec<CallFrame>, // Call stack
    current_frame: Option<usize>, // Index of current call frame
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            ip: 0,
            stack: Vec::new(),
            variables: HashMap::new(),
            frames: Vec::new(),
            current_frame: None,
        };
        
        // Register native functions
        vm.register_native_functions();
        
        vm
    }
    
    // Register built-in native functions
    fn register_native_functions(&mut self) {
        self.define_native("print_value", 1, VM::native_print_value);
    }
    
    // Define a native function
    fn define_native(&mut self, name: &str, arity: u8, function: fn(&[Value]) -> Result<Value, String>) {
        let native_fn = Value::NativeFunction(NativeFunction {
            name: name.to_string(),
            arity,
            function,
        });
        
        self.variables.insert(name.to_string(), native_fn);
    }
    
    // Native function implementation: print_value
    fn native_print_value(args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("print_value expects exactly 1 argument".to_string());
        }
        
        println!("{}", args[0]);
        
        // Return nil or some default value
        Ok(Value::I32(0))
    }
    
    pub fn get_variables(&self) -> &HashMap<String, Value> {
        &self.variables
    }
    
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.ip = 0;
        self.stack.clear();
        self.variables.clear();
        self.frames.clear();
        self.current_frame = None;
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), String> {
        self.ip = 0;
        while self.ip < chunk.code.len() {
            self.execute_instruction(chunk)?;
        }

        // Print all values left on the stack when the program ends for debugging
        #[cfg(feature = "trace-execution")]
        {
            if !self.stack.is_empty() {
                println!("\n=== Values on stack at end of execution ===");
                for value in &self.stack {
                    println!("{}", value);
                }
            }
        }

        Ok(())
    }

    fn execute_instruction(&mut self, chunk: &Chunk) -> Result<(), String> {
        let instruction = self.read_byte(chunk);
        let op = OpCode::from_u8(instruction)
            .ok_or_else(|| format!("Unknown opcode: {}", instruction))?;

        match op {
            OpCode::Constant => {
                let constant_idx = self.read_byte(chunk) as usize;
                if constant_idx >= chunk.constants.len() {
                    return Err("Invalid constant index".to_string());
                }
                let constant = chunk.constants[constant_idx].clone();
                self.stack.push(constant);
            }
            OpCode::Add => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => Ok(Value::I32(*a + *b)),
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(*a + *b)),
                    (Value::U32(a), Value::U32(b)) => Ok(Value::U32(*a + *b)),
                    (Value::U64(a), Value::U64(b)) => Ok(Value::U64(*a + *b)),
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a + *b)),
                    (Value::String(a), Value::String(b)) => {
                        Ok(Value::String(format!("{}{}", a, b)))
                    }
                    _ => Err("Cannot add these types".to_string()),
                })?;
            }
            OpCode::Subtract => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => Ok(Value::I32(*a - *b)),
                    (Value::I64(a), Value::I64(b)) => Ok(Value::I64(*a - *b)),
                    (Value::U32(a), Value::U32(b)) => {
                        if *a >= *b {
                            Ok(Value::U32(*a - *b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::U64(a), Value::U64(b)) => {
                        if *a >= *b {
                            Ok(Value::U64(*a - *b))
                        } else {
                            Err("Unsigned underflow".to_string())
                        }
                    }
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a - *b)),
                    _ => Err("Cannot subtract these types".to_string()),
                })?;
            }
            OpCode::Multiply => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => match (*a).checked_mul(*b) {
                        Some(result) => Ok(Value::I32(result)),
                        None => Err("Integer overflow in I32 multiplication".to_string()),
                    },
                    (Value::I64(a), Value::I64(b)) => match (*a).checked_mul(*b) {
                        Some(result) => Ok(Value::I64(result)),
                        None => Err("Integer overflow in I64 multiplication".to_string()),
                    },
                    (Value::U32(a), Value::U32(b)) => match (*a).checked_mul(*b) {
                        Some(result) => Ok(Value::U32(result)),
                        None => Err("Integer overflow in U32 multiplication".to_string()),
                    },
                    (Value::U64(a), Value::U64(b)) => match (*a).checked_mul(*b) {
                        Some(result) => Ok(Value::U64(result)),
                        None => Err("Integer overflow in U64 multiplication".to_string()),
                    },
                    (Value::F64(a), Value::F64(b)) => Ok(Value::F64(*a * *b)),
                    _ => Err("Cannot multiply these types".to_string()),
                })?;
            }
            OpCode::Divide => {
                self.binary_op(|a, b| match (a, b) {
                    (Value::I32(a), Value::I32(b)) => {
                        if *b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::I32(*a / *b))
                    }
                    (Value::I64(a), Value::I64(b)) => {
                        if *b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::I64(*a / *b))
                    }
                    (Value::U32(a), Value::U32(b)) => {
                        if *b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::U32(*a / *b))
                    }
                    (Value::U64(a), Value::U64(b)) => {
                        if *b == 0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::U64(*a / *b))
                    }
                    (Value::F64(a), Value::F64(b)) => {
                        if *b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::F64(*a / *b))
                    }
                    _ => Err("Cannot divide these types".to_string()),
                })?;
            }
            OpCode::Negate => {
                let value = self.pop()?;
                match value {
                    Value::I32(i) => self.stack.push(Value::I32(-i)),
                    Value::I64(i) => self.stack.push(Value::I64(-i)),
                    Value::U32(_) => return Err("Cannot negate unsigned integer U32".to_string()),
                    Value::U64(_) => return Err("Cannot negate unsigned integer U64".to_string()),
                    Value::F64(f) => self.stack.push(Value::F64(-f)),
                    _ => return Err("Can only negate numbers".to_string()),
                }
            }
            OpCode::Return => {
                // If we're in a function, return to the caller
                if let Some(frame_index) = self.current_frame {
                    let return_value = if self.stack.is_empty() {
                        Value::I32(0) // Default return value
                    } else {
                        self.pop()?
                    };
                    
                    let frame = &self.frames[frame_index];
                    let return_address = frame.return_address;
                    let stack_offset = frame.stack_offset;
                    
                    // Clear stack to the previous position, keeping only items below the stack offset
                    while self.stack.len() > stack_offset {
                        self.pop()?;
                    }
                    
                    // Push return value
                    self.stack.push(return_value);
                    
                    // Return to caller
                    self.ip = return_address;
                    
                    // Pop the frame
                    self.frames.pop();
                    self.current_frame = if self.frames.is_empty() {
                        None
                    } else {
                        Some(self.frames.len() - 1)
                    };
                } else {
                    // If we're not in a function, end the program
                    self.ip = chunk.code.len();
                }
            }
            OpCode::Print => {
                let value = self.pop()?;
                println!("{}", value);
            }
            OpCode::GetVariable => {
                let var_index = self.read_byte(chunk) as usize;
                if var_index >= chunk.identifiers.len() {
                    return Err("Invalid variable index".to_string());
                }
                let var_name = &chunk.identifiers[var_index];
                
                // First check if this is a local variable in the current function
                let value = if let Some(frame_idx) = self.current_frame {
                    if let Some(value) = self.frames[frame_idx].locals.get(var_name) {
                        value.clone()
                    } else if let Some(value) = self.variables.get(var_name) {
                        value.clone()
                    } else {
                        return Err(format!("Undefined variable '{}'", var_name));
                    }
                } else {
                    // Global scope
                    if let Some(value) = self.variables.get(var_name) {
                        value.clone()
                    } else {
                        return Err(format!("Undefined variable '{}'", var_name));
                    }
                };
                
                self.stack.push(value);
            }
            OpCode::SetVariable => {
                if self.stack.is_empty() {
                    return Err("Stack underflow".to_string());
                }
                let var_index = self.read_byte(chunk) as usize;
                if var_index >= chunk.identifiers.len() {
                    return Err("Invalid variable index".to_string());
                }
                let var_name = chunk.identifiers[var_index].clone();
                let value = self.stack.last().unwrap().clone();
                
                // If we're in a function, first check if this is a local variable
                if let Some(frame_idx) = self.current_frame {
                    if self.frames[frame_idx].function.locals.contains(&var_name) {
                        // It's a local variable, update it in the current frame
                        self.frames[frame_idx].locals.insert(var_name, value);
                    } else {
                        // Not a local, update global
                        self.variables.insert(var_name, value);
                    }
                } else {
                    // Global scope
                    self.variables.insert(var_name, value);
                }
            }
            OpCode::Pop => {
                self.pop()?;
            }
            OpCode::DefineFunction => {
                let var_index = self.read_byte(chunk) as usize;
                let constant_index = self.read_byte(chunk) as usize;
                
                if var_index >= chunk.identifiers.len() || constant_index >= chunk.constants.len() {
                    return Err("Invalid index for function definition".to_string());
                }
                
                let var_name = chunk.identifiers[var_index].clone();
                let value = chunk.constants[constant_index].clone();
                
                // Store function in variables
                self.variables.insert(var_name, value);
            }
            OpCode::Call => {
                let arg_count = self.read_byte(chunk) as usize;
                
                // Calculate the function position correctly
                if self.stack.len() < arg_count + 1 {
                    return Err("Stack underflow during function call".to_string());
                }
                
                // Save the function object
                let function_pos = self.stack.len() - 1;
                let function_value = self.stack[function_pos].clone();
                
                // Handle the call based on the function type
                match function_value {
                    Value::Function(func) => {
                        // Check argument count
                        if arg_count != func.arity as usize {
                            return Err(format!(
                                "Expected {} arguments but got {}",
                                func.arity, arg_count
                            ));
                        }
                        
                        // Create local variables map for the function parameters
                        let mut locals = HashMap::new();
                        
                        // Set up parameter values from arguments
                        for i in 0..arg_count {
                            if i < func.locals.len() {
                                // Get parameter name from function locals
                                let param_name = &func.locals[i];
                                
                                // Get argument value from stack
                                let arg_value = self.stack[function_pos + 1 + i].clone();
                                
                                // Store as a local variable
                                locals.insert(param_name.clone(), arg_value);
                            }
                        }
                        
                        // Create a new call frame
                        let frame = CallFrame {
                            function: func.clone(),
                            return_address: self.ip,
                            stack_offset: function_pos,
                            locals,
                        };
                        
                        // Store the frame and update current frame
                        self.frames.push(frame);
                        self.current_frame = Some(self.frames.len() - 1);
                        
                        // Jump to function code
                        self.ip = func.code_offset;
                    },
                    Value::NativeFunction(native_fn) => {
                        // Check argument count
                        if arg_count != native_fn.arity as usize {
                            return Err(format!(
                                "Expected {} arguments but got {}",
                                native_fn.arity, arg_count
                            ));
                        }
                        
                        // Collect arguments from stack
                        let mut args = Vec::with_capacity(arg_count);
                        for i in 0..arg_count {
                            args.push(self.stack[function_pos - 1 - i].clone());
                        }
                        
                        // Call the native function
                        let result = (native_fn.function)(&args)?;
                        
                        // Remove function and arguments from stack
                        for _ in 0..=arg_count {
                            self.pop()?;
                        }
                        
                        // Push result onto the stack
                        self.stack.push(result);
                    },
                    _ => return Err("Can only call functions".to_string()),
                }
            }
            OpCode::Jump => {
                let offset = ((self.read_byte(chunk) as usize) << 8) | self.read_byte(chunk) as usize;
                self.ip += offset;
            }
            OpCode::JumpIfFalse => {
                let offset = ((self.read_byte(chunk) as usize) << 8) | self.read_byte(chunk) as usize;
                let condition = self.peek(0)?;
                
                // Determine truthiness of the condition
                let is_truthy = match condition {
                    Value::I32(i) => *i != 0,
                    Value::I64(i) => *i != 0,
                    Value::U32(i) => *i != 0,
                    Value::U64(i) => *i != 0,
                    Value::F64(f) => *f != 0.0,
                    Value::String(s) => !s.is_empty(),
                    Value::Function(_) => true,
                    Value::NativeFunction(_) => true,
                };
                
                if !is_truthy {
                    self.ip += offset;
                }
            }
        }

        Ok(())
    }

    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    fn pop(&mut self) -> Result<Value, String> {
        self.stack
            .pop()
            .ok_or_else(|| "Stack underflow".to_string())
    }
    
    fn peek(&self, distance: usize) -> Result<&Value, String> {
        if distance >= self.stack.len() {
            return Err("Stack underflow".to_string());
        }
        
        Ok(&self.stack[self.stack.len() - 1 - distance])
    }

    fn binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: FnOnce(&Value, &Value) -> Result<Value, String>,
    {
        if self.stack.len() < 2 {
            return Err("Stack underflow".to_string());
        }
        let b = self.pop()?;
        let a = self.pop()?;
        let result = op(&a, &b)?;
        self.stack.push(result);
        Ok(())
    }
}
