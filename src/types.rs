use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::cell::RefCell;

/// A unique identifier for a type in the type system
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(usize);

impl TypeId {
    /// Creates a new unique type identifier
    pub fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        TypeId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Returns the TypeId for booleans
pub fn bool_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("bool").unwrap().clone())
}

/// Returns the TypeId for i32 integers
pub fn i32_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i32").unwrap().clone())
}

/// Returns the TypeId for i64 integers
pub fn i64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i64").unwrap().clone())
}

/// Returns the TypeId for u32 integers
pub fn u32_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u32").unwrap().clone())
}

/// Returns the TypeId for u64 integers
pub fn u64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u64").unwrap().clone())
}

/// Returns the TypeId for f32 floating points
pub fn f32_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f32").unwrap().clone())
}

/// Returns the TypeId for f64 floating points
pub fn f64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f64").unwrap().clone())
}

/// Returns the TypeId for unspecified floats (used for float literals without suffix)
pub fn unspecified_float_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("float").unwrap().clone())
}

/// Returns the TypeId for strings
pub fn string_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("string").unwrap().clone())
}

/// Returns the TypeId for unspecified integers (used for integer literals)
pub fn unspecified_int_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("int").unwrap().clone())
}

/// Returns the TypeId for unknown types
pub fn unknown_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("unknown").unwrap().clone())
}

/// Represents the different kinds of types in the language
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TypeKind {
    /// Integer types (signed/unsigned, different bit widths)
    Integer(IntegerType),
    /// Floating point types
    Float(FloatType),
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Struct type with fields
    Struct(StructType),
    /// Unknown or not yet determined type
    Unknown,
}

/// Represents an integer type with its properties
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IntegerType {
    /// Whether the integer is signed or unsigned
    pub signed: bool,
    /// The number of bits (e.g., 32 for i32)
    pub bits: u8,
    /// Whether this is an unspecified integer (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a floating point type with its properties
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FloatType {
    /// The number of bits (e.g., 64 for f64)
    pub bits: u8,
    /// Whether this is an unspecified float (used for literals without explicit type)
    pub is_unspecified: bool,
}

/// Represents a struct type with its fields
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StructType {
    /// Name of the struct
    pub name: String,
    /// Fields of the struct with their names and types
    pub fields: Vec<(String, TypeId)>,
}

/// Contains all information about a specific type
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TypeInfo {
    /// Unique identifier for this type
    pub id: TypeId,
    /// Name of the type
    pub name: String,
    /// The kind of type (integer, float, string, etc.)
    pub kind: TypeKind,
}


// Thread-local storage for the type registry and commonly used types
thread_local! {
    /// The global type registry
    pub static TYPE_REGISTRY: RefCell<TypeRegistry> = RefCell::new(TypeRegistry::new());

    /// Pre-defined type for booleans
    pub static BOOL_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("bool").unwrap().clone()));
    /// Pre-defined type for i32 integers
    pub static I32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i32").unwrap().clone()));
    /// Pre-defined type for i64 integers
    pub static I64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i64").unwrap().clone()));
    /// Pre-defined type for u32 integers
    pub static U32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u32").unwrap().clone()));
    /// Pre-defined type for u64 integers
    pub static U64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u64").unwrap().clone()));
    /// Pre-defined type for f32 floating points
    pub static F32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f32").unwrap().clone()));
    /// Pre-defined type for f64 floating points
    pub static F64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f64").unwrap().clone()));
    /// Pre-defined type for strings
    pub static STRING_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("string").unwrap().clone()));
    /// Pre-defined type for unspecified integers (used for integer literals)
    pub static UNSPECIFIED_INT_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("int").unwrap().clone()));
    /// Pre-defined type for unspecified floats (used for float literals without suffix)
    pub static UNSPECIFIED_FLOAT_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("float").unwrap().clone()));
    /// Pre-defined type for unknown types
    pub static UNKNOWN_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("unknown").unwrap().clone()));
}

/// Registry that stores all available types in the language
pub struct TypeRegistry {
    /// Map from TypeId to TypeInfo
    types: HashMap<TypeId, TypeInfo>,
    /// Map from type names to TypeId for quick lookup
    type_names: HashMap<String, TypeId>,
}


impl TypeRegistry {
    /// Creates a new TypeRegistry with built-in types registered
    pub fn new() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
            type_names: HashMap::new(),
        };

        // Register the built-in types
        registry.register_built_in_types();
        registry
    }

    /// Registers all built-in types in the type registry
    fn register_built_in_types(&mut self) {
        self.register_type("i32", TypeKind::Integer(IntegerType { signed: true, bits: 32, is_unspecified: false }));
        self.register_type("i64", TypeKind::Integer(IntegerType { signed: true, bits: 64, is_unspecified: false }));
        self.register_type("u32", TypeKind::Integer(IntegerType { signed: false, bits: 32, is_unspecified: false }));
        self.register_type("u64", TypeKind::Integer(IntegerType { signed: false, bits: 64, is_unspecified: false }));
        self.register_type("int", TypeKind::Integer(IntegerType { signed: true, bits: 0, is_unspecified: true }));
        
        self.register_type("f32", TypeKind::Float(FloatType { bits: 32, is_unspecified: false }));
        self.register_type("f64", TypeKind::Float(FloatType { bits: 64, is_unspecified: false }));
        self.register_type("float", TypeKind::Float(FloatType { bits: 0, is_unspecified: true }));
        
        self.register_type("string", TypeKind::String);
        self.register_type("bool", TypeKind::Boolean);
        
        self.register_type("unknown", TypeKind::Unknown);
    }

    /// Registers a new type in the registry
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the type
    /// * `kind` - The kind of type to register
    /// 
    /// # Returns
    /// 
    /// The TypeId assigned to the newly registered type
    pub fn register_type(&mut self, name: &str, kind: TypeKind) -> TypeId {
        let id = TypeId::new();
        let type_info = TypeInfo {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };
        
        self.types.insert(id.clone(), type_info);
        self.type_names.insert(name.to_string(), id.clone());
        
        id
    }

    /// Looks up a type by name
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the type to look up
    /// 
    /// # Returns
    /// 
    /// Some(TypeId) if found, None otherwise
    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeId> {
        self.type_names.get(name)
    }
 
    /// Gets type information for a given TypeId
    /// 
    /// # Arguments
    /// 
    /// * `id` - The TypeId to look up
    /// 
    /// # Returns
    /// 
    /// Some(TypeInfo) if found, None otherwise
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.types.get(id)
    }

    /// Checks if a value is within the valid range for a given type
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to check
    /// * `type_id` - The type to check against
    /// 
    /// # Returns
    /// 
    /// true if the value is in range, false otherwise
    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };
        
        match &type_info.kind {
            TypeKind::Integer(int_type) => {
                match (int_type.signed, int_type.bits) {
                    (true, 32) => *value >= i32::MIN as i64 && *value <= i32::MAX as i64,
                    (true, 64) => true, // all i64 fit in i64
                    (false, 32) => *value >= 0 && *value <= u32::MAX as i64,
                    (false, 64) => *value >= 0, // all positive i64 fit in u64
                    _ => false,
                }
            },
            _ => false,
        }
    }

    /// Checks if a float value is within the valid range for a given type
    /// 
    /// # Arguments
    /// 
    /// * `value` - The value to check
    /// * `type_id` - The type to check against
    /// 
    /// # Returns
    /// 
    /// true if the value is in range, false otherwise
    pub fn check_float_value_in_range(&self, value: &f64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };
        
        match &type_info.kind {
            TypeKind::Float(float_type) => {
                match float_type.bits {
                    32 => *value >= f32::MIN as f64 && *value <= f32::MAX as f64,
                    64 => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}