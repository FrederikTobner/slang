use std::cell::RefCell;
use std::fmt::Debug;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeId(usize);

impl TypeId {
    fn new() -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        TypeId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TypeKind {
    Integer(IntegerType),
    Float(FloatType),
    String,
    Struct(StructType),
    Unknown,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IntegerType {
    pub signed: bool,
    pub bits: u8,
    pub is_unspecified: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FloatType {
    pub bits: u8,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StructType {
    pub name: String,
    pub fields: Vec<(String, TypeId)>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TypeInfo {
    pub id: TypeId,
    pub name: String,
    pub kind: TypeKind,
}

pub struct TypeRegistry {
    types: HashMap<TypeId, TypeInfo>,
    type_names: HashMap<String, TypeId>,
}

thread_local! {
    pub static TYPE_REGISTRY: RefCell<TypeRegistry> = RefCell::new(TypeRegistry::new());

    pub static I32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i32").unwrap().clone()));
    pub static I64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i64").unwrap().clone()));
    pub static U32_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u32").unwrap().clone()));
    pub static U64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u64").unwrap().clone()));
    pub static F64_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f64").unwrap().clone()));
    pub static STRING_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("string").unwrap().clone()));
    pub static UNSPECIFIED_INT_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("int").unwrap().clone()));
    pub static UNKNOWN_TYPE: RefCell<TypeId> = RefCell::new(TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("unknown").unwrap().clone()));
}

// Add accessor functions
pub fn i32_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i32").unwrap().clone())
}

pub fn i64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("i64").unwrap().clone())
}

pub fn u32_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u32").unwrap().clone())
}

pub fn u64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("u64").unwrap().clone())
}

pub fn f64_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("f64").unwrap().clone())
}

pub fn string_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("string").unwrap().clone())
}

pub fn unspecified_int_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("int").unwrap().clone())
}

pub fn unknown_type() -> TypeId {
    TYPE_REGISTRY.with(|r| r.borrow().get_type_by_name("unknown").unwrap().clone())
}

impl TypeRegistry {
    pub fn new() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
            type_names: HashMap::new(),
        };

        // Register the built-in types
        registry.register_built_in_types();
        registry
    }

    fn register_built_in_types(&mut self) {
        self.register_type("i32", TypeKind::Integer(IntegerType { signed: true, bits: 32, is_unspecified: false }));
        self.register_type("i64", TypeKind::Integer(IntegerType { signed: true, bits: 64, is_unspecified: false }));
        self.register_type("u32", TypeKind::Integer(IntegerType { signed: false, bits: 32, is_unspecified: false }));
        self.register_type("u64", TypeKind::Integer(IntegerType { signed: false, bits: 64, is_unspecified: false }));
        self.register_type("int", TypeKind::Integer(IntegerType { signed: true, bits: 0, is_unspecified: true }));
        
        self.register_type("f64", TypeKind::Float(FloatType { bits: 64 }));
        
        self.register_type("string", TypeKind::String);
        
        self.register_type("unknown", TypeKind::Unknown);
    }

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

    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeId> {
        self.type_names.get(name)
    }
 
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.types.get(id)
    }

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
}