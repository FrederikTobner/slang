use std::collections::HashMap;
use std::sync::RwLock;
use crate::types::{TypeId, TypeKind, IntegerType, FloatType};

#[derive(Debug, Clone)]
pub struct Type {
    pub id: TypeId,
    pub name: String,
    pub kind: TypeKind,
}

pub struct TypeRegistry {
    types: HashMap<TypeId, Type>,
    // Map type names to their IDs for lookup
    type_names: HashMap<String, TypeId>,
    // Track subtyping/coercion relationships
    coercion_rules: HashMap<TypeId, Vec<TypeId>>,
}

// Standard built-in types
lazy_static::lazy_static! {
    pub static ref I32_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_i32_type());
    pub static ref I64_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_i64_type());
    pub static ref U32_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_u32_type());
    pub static ref U64_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_u64_type());
    pub static ref F64_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_f64_type());
    pub static ref STRING_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_string_type());
    pub static ref UNSPECIFIED_INT_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_unspecified_int_type());
    pub static ref UNKNOWN_TYPE: TypeId = TYPE_REGISTRY.with(|r| r.borrow().get_unknown_type());
}

// Thread-local registry
thread_local! {
    pub static TYPE_REGISTRY: std::cell::RefCell<TypeRegistry> = std::cell::RefCell::new(TypeRegistry::new());
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
            type_names: HashMap::new(),
            coercion_rules: HashMap::new(),
        };

        // Register the built-in types
        registry.register_built_in_types();
        registry
    }

    pub fn register_built_in_types(&mut self) {
        // Register integer types
        let i32_type = self.register_type("i32", TypeKind::Integer(IntegerType { signed: true, bits: 32 }));
        let i64_type = self.register_type("i64", TypeKind::Integer(IntegerType { signed: true, bits: 64 }));
        let u32_type = self.register_type("u32", TypeKind::Integer(IntegerType { signed: false, bits: 32 }));
        let u64_type = self.register_type("u64", TypeKind::Integer(IntegerType { signed: false, bits: 64 }));
        let unspecified_int = self.register_type("int", TypeKind::Integer(IntegerType { signed: true, bits: 0 }));

        // Register float type
        let f64_type = self.register_type("f64", TypeKind::Float(FloatType { bits: 64 }));

        // Register string type
        let string_type = self.register_type("string", TypeKind::String);

        // Register unknown type
        let unknown_type = self.register_type("unknown", TypeKind::Unknown);

        // Define coercion rules for unspecified integer
        self.add_coercion_rule(unspecified_int, i32_type);
        self.add_coercion_rule(unspecified_int, i64_type);
        self.add_coercion_rule(unspecified_int, u32_type);
        self.add_coercion_rule(unspecified_int, u64_type);
        
        // Add more coercion rules as needed
        self.add_coercion_rule(i32_type, i64_type);
        self.add_coercion_rule(u32_type, u64_type);
    }

    // Register a new type and return its ID
    pub fn register_type(&mut self, name: &str, kind: TypeKind) -> TypeId {
        let id = TypeId::new();
        let type_obj = Type {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };
        
        self.types.insert(id.clone(), type_obj);
        self.type_names.insert(name.to_string(), id.clone());
        
        id
    }

    // Look up a type by name
    pub fn get_type_by_name(&self, name: &str) -> Option<TypeId> {
        self.type_names.get(name).cloned()
    }

    // Look up type information 
    pub fn get_type_info(&self, id: &TypeId) -> Option<&Type> {
        self.types.get(id)
    }

    // Add a coercion rule (from_type can be coerced to to_type)
    pub fn add_coercion_rule(&mut self, from_type: TypeId, to_type: TypeId) {
        self.coercion_rules
            .entry(from_type)
            .or_insert_with(Vec::new)
            .push(to_type);
    }

    // Check if one type can be coerced to another
    pub fn can_coerce(&self, from_type: &TypeId, to_type: &TypeId) -> bool {
        // Same types are always compatible
        if from_type == to_type {
            return true;
        }
        
        // Check direct coercion rules
        if let Some(targets) = self.coercion_rules.get(from_type) {
            if targets.contains(to_type) {
                return true;
            }
        }
        
        // Special case for unspecified integer - need value-dependent check
        if from_type == self.get_unspecified_int_type() {
            // In actual use, you'd need the value to check range
            return matches!(
                self.get_type_info(to_type).map(|t| &t.kind),
                Some(TypeKind::Integer(_))
            );
        }
        
        false
    }

    // Value-dependent coercion check
    pub fn can_coerce_value(&self, from_type: &TypeId, to_type: &TypeId, value: &crate::ast::Value) -> bool {
        if let crate::ast::Value::UnspecifiedInteger(n) = value {
            // Check range constraints based on target type
            if let Some(type_info) = self.get_type_info(to_type) {
                match &type_info.kind {
                    TypeKind::Integer(int_type) => {
                        match (int_type.signed, int_type.bits) {
                            (true, 32) => *n >= i32::MIN as i64 && *n <= i32::MAX as i64,
                            (true, 64) => true, // all i64 fit in i64
                            (false, 32) => *n >= 0 && *n <= u32::MAX as i64,
                            (false, 64) => *n >= 0, // all positive i64 fit in u64
                            _ => false,
                        }
                    },
                    _ => false,
                }
            } else {
                false
            }
        } else {
            // For non-UnspecifiedInteger values, fall back to regular coercion rules
            self.can_coerce(from_type, to_type)
        }
    }

    // Accessor methods for built-in types
    pub fn get_i32_type(&self) -> TypeId {
        self.type_names.get("i32").unwrap().clone()
    }
    
    pub fn get_i64_type(&self) -> TypeId {
        self.type_names.get("i64").unwrap().clone()
    }
    
    pub fn get_u32_type(&self) -> TypeId {
        self.type_names.get("u32").unwrap().clone()
    }
    
    pub fn get_u64_type(&self) -> TypeId {
        self.type_names.get("u64").unwrap().clone()
    }
    
    pub fn get_f64_type(&self) -> TypeId {
        self.type_names.get("f64").unwrap().clone()
    }
    
    pub fn get_string_type(&self) -> TypeId {
        self.type_names.get("string").unwrap().clone()
    }
    
    pub fn get_unspecified_int_type(&self) -> TypeId {
        self.type_names.get("int").unwrap().clone()
    }
    
    pub fn get_unknown_type(&self) -> TypeId {
        self.type_names.get("unknown").unwrap().clone()
    }
}