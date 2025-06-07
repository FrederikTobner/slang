pub mod registry;
pub mod types;

pub use registry::TypeRegistry;
pub use types::{PrimitiveType, StructType, TypeId, TypeInfo, TypeKind};
pub use types::{
    TYPE_NAME_BOOL, TYPE_NAME_F32, TYPE_NAME_F64, TYPE_NAME_FLOAT, TYPE_NAME_I32, TYPE_NAME_I64,
    TYPE_NAME_INT, TYPE_NAME_STRING, TYPE_NAME_U32, TYPE_NAME_U64, TYPE_NAME_UNIT,
    TYPE_NAME_UNKNOWN,
};
