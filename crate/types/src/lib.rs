pub mod registry;
pub mod inference;

// Core type system modules
pub mod type_id;
pub mod primitive;
pub mod kind;
pub mod info;

pub use registry::TypeRegistry;
pub use type_id::TypeId;
pub use primitive::{
    PrimitiveType,
    TYPE_NAME_BOOL, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_INT, TYPE_NAME_STRING, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNIT,
    TYPE_NAME_UNKNOWN,
};
pub use kind::{TypeKind, IntegerType, FloatType, StructType, FunctionType};
pub use info::TypeInfo;
pub use inference::{TypePromotion, TypeQueries, TypeOperations};
