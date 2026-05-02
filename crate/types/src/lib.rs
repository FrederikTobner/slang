pub mod inference;
pub mod registry;

// Core type system modules
pub mod info;
pub mod kind;
pub mod primitive;
pub mod type_id;

pub use inference::{TypeOperations, TypePromotion, TypeQueries};
pub use info::TypeInfo;
pub use kind::{FloatType, FunctionType, IntegerType, StructType, TypeKind};
pub use primitive::{
    PrimitiveType, TYPE_NAME_BOOL, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32,
    TYPE_NAME_I64, TYPE_NAME_INT, TYPE_NAME_STRING, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNIT,
    TYPE_NAME_UNKNOWN,
};
pub use registry::TypeRegistry;
pub use type_id::TypeId;
