use crate::bytecode::{Chunk, Function, NativeFunction, OpCode, Value};
use std::collections::HashMap;

/// Call frame to track function calls
struct CallFrame {
    /// The function being called
    function: Function,
    /// Address to return to after function completes
    return_address: usize,
    /// Stack position before function call
    stack_offset: usize,
    /// Local variables for the function
    locals: HashMap<String, Value>,
}

/// Virtual Machine that executes bytecode
pub struct VM {
    /// Instruction pointer
    ip: usize, 
    /// Stack for values
    stack: Vec<Value>,
    /// Global variables
    variables: HashMap<String, Value>,
    /// Call frames for function calls
    frames: Vec<CallFrame>, 
    /// Index of the current call frame
    current_frame: Option<usize>, 
}

impl VM {
    /// Creates a new virtual machine
    pub fn new() -> Self {
        let mut vm = VM {
            ip: 0,
            stack: Vec::new(),
            variables: HashMap::new(),
            frames: Vec::new(),
            current_frame: None,
        };

        vm.register_native_functions();
        
        vm
    }
    
    /// Registers built-in functions
    fn register_native_functions(&mut self) {
        self.define_native("print_value", 1, VM::native_print_value);
    }
    
    /// Defines a native (built-in) function
    /// 
    /// # Arguments
    /// 
    /// * `name` - Name of the native function
    /// * `arity` - Number of parameters
    /// * `function` - The Rust function implementing this native function
    fn define_native(&mut self, name: &str, arity: u8, function: fn(&[Value]) -> Result<Value, String>) {
        let native_fn = Value::NativeFunction(NativeFunction {
            name: name.to_string(),
            arity,
            function,
        });
        
        self.variables.insert(name.to_string(), native_fn);
    }
    
    /// Built-in function to print a value
    /// 
    /// # Arguments
    /// 
    /// * `args` - Arguments to the function (should be exactly 1)
    /// 
    /// # Returns
    /// 
    /// Success with i32(0) if successful, or an error message
    fn native_print_value(args: &[Value]) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("print_value expects exactly 1 argument".to_string());
        }
        
        println!("{}", args[0]);
        
        // Return 0 to indicate success
        Ok(Value::I32(0))
    }

    /// Interprets and executes a bytecode chunk
    /// 
    /// # Arguments
    /// 
    /// * `chunk` - The bytecode chunk to execute
    /// 
    /// # Returns
    /// 
    /// Ok(()) on success, or an error message on failure
    pub fn interpret(&mut self, chunk: &Chunk) -> Result<(), String> {
        self.ip = 0;
        while self.ip < chunk.code.len() {
            self.execute_instruction(chunk)?;
        }

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

    /// Executes a single instruction
    /// 
    /// # Arguments
    /// 
    /// * `chunk` - The bytecode chunk containing the instruction
    /// 
    /// # Returns
    /// 
    /// Ok(()) on success, or an error message on failure
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
                    (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a + *b)),
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
                    (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a - *b)),
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
                    (Value::F32(a), Value::F32(b)) => Ok(Value::F32(*a * *b)),
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
                    (Value::F32(a), Value::F32(b)) => {
                        if *b == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        Ok(Value::F32(*a / *b))
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
                    Value::F32(f) => self.stack.push(Value::F32(-f)),
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
                    
                    while self.stack.len() > stack_offset {
                        self.pop()?;
                    }
                    
                    self.stack.push(return_value);
                    
                    self.ip = return_address;
                    
                    self.frames.pop();
                    self.current_frame = if self.frames.is_empty() {
                        None
                    } else {
                        Some(self.frames.len() - 1)
                    };
                } else {
                    // If we're not in a function, stop execution
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
                
                let value = if let Some(frame_idx) = self.current_frame {
                    if let Some(value) = self.frames[frame_idx].locals.get(var_name) {
                        value.clone()
                    } else if let Some(value) = self.variables.get(var_name) {
                        value.clone()
                    } else {
                        return Err(format!("Undefined variable '{}'", var_name));
                    }
                } else {
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
                
                if let Some(frame_idx) = self.current_frame {
                    if self.frames[frame_idx].function.locals.contains(&var_name) {
                        // Local variable
                        self.frames[frame_idx].locals.insert(var_name, value);
                    } else {
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

                self.variables.insert(var_name, value);
            }
            OpCode::Call => {
                let arg_count = self.read_byte(chunk) as usize;

                if self.stack.len() < arg_count + 1 {
                    return Err("Stack underflow during function call".to_string());
                }
                
                let function_pos = self.stack.len() - 1;
                let function_value = self.stack[function_pos].clone();
                
                match function_value {
                    Value::Function(func) => {
                        // Check argument count
                        if arg_count != func.arity as usize {
                            return Err(format!(
                                "Expected {} arguments but got {}",
                                func.arity, arg_count
                            ));
                        }
                        
                        let mut locals = HashMap::new();

                        for i in 0..arg_count {
                            if i < func.locals.len() {
                                let param_name = &func.locals[arg_count - 1 - i];
                                let arg_value = self.stack[function_pos - 1 - i].clone();
                                locals.insert(param_name.clone(), arg_value);
                            }
                        }
                        
                        let frame = CallFrame {
                            function: func.clone(),
                            return_address: self.ip,
                            stack_offset: function_pos,
                            locals,
                        };
                        
                        self.frames.push(frame);
                        self.current_frame = Some(self.frames.len() - 1);
                        
                        self.ip = func.code_offset;
                    },
                    Value::NativeFunction(native_fn) => {
                        if arg_count != native_fn.arity as usize {
                            return Err(format!(
                                "Expected {} arguments but got {}",
                                native_fn.arity, arg_count
                            ));
                        }
                        
                        let mut args = Vec::with_capacity(arg_count);
                        for i in 0..arg_count {
                            args.push(self.stack[function_pos - 1 - i].clone());
                        }
                        
                        let result = (native_fn.function)(&args)?;
                        for _ in 0..=arg_count {
                            self.pop()?;
                        }
                        
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
                
                let is_truthy = match condition {
                    Value::Boolean(b) => *b,
                    _ => return Err("Condition must be a boolean".to_string()),
                };
                
                if !is_truthy {
                    self.ip += offset;
                }
            }
            OpCode::BoolNot => {
                let value = self.pop()?;
                match value {
                    Value::Boolean(b) => self.stack.push(Value::Boolean(!b)),
                    _ => return Err("Can only negate boolean values".to_string()),
                }
            }
        }

        Ok(())
    }

    /// Reads the next byte from the chunk and advances the instruction pointer
    /// 
    /// # Arguments
    /// 
    /// * `chunk` - The bytecode chunk to read from
    /// 
    /// # Returns
    /// 
    /// The byte read from the chunk
    fn read_byte(&mut self, chunk: &Chunk) -> u8 {
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }

    /// Pops a value off the stack
    /// 
    /// # Returns
    /// 
    /// The popped value, or an error if the stack is empty
    fn pop(&mut self) -> Result<Value, String> {
        self.stack
            .pop()
            .ok_or_else(|| "Stack underflow".to_string())
    }
    
    /// Looks at a value on the stack without removing it
    /// 
    /// # Arguments
    /// 
    /// * `distance` - How far from the top of the stack to look
    /// 
    /// # Returns
    /// 
    /// Reference to the value, or an error if the stack isn't deep enough
    fn peek(&self, distance: usize) -> Result<&Value, String> {
        if distance >= self.stack.len() {
            return Err("Stack underflow".to_string());
        }
        
        Ok(&self.stack[self.stack.len() - 1 - distance])
    }

    /// Performs a binary operation on the top two values of the stack
    /// 
    /// # Arguments
    /// 
    /// * `op` - Function that implements the binary operation
    /// 
    /// # Returns
    /// 
    /// Ok(()) if successful, or an error message
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
