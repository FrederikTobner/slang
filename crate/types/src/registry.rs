use std::collections::HashMap;
use crate::{PrimitiveType, TypeId, TypeInfo, TypeKind};
use crate::types::{IntegerType, FloatType};

/// Registry that stores all available types in the language
pub struct TypeRegistry {
    /// Map from TypeId to TypeInfo
    types: HashMap<TypeId, TypeInfo>,
}

impl TypeRegistry {
    /// Creates a new TypeRegistry with built-in types registered.
    pub fn new_instance() -> Self {
        let mut registry = TypeRegistry {
            types: HashMap::new(),
        };
        registry.register_built_in_types();
        registry
    }

    /// Registers all built-in types in the type registry
    fn register_built_in_types(&mut self) {
        let types_to_register: &[(PrimitiveType, TypeKind)] = &[
            (
                PrimitiveType::I32,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::I64,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::U32,
                TypeKind::Integer(IntegerType {
                    signed: false,
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::U64,
                TypeKind::Integer(IntegerType {
                    signed: false,
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::UnspecifiedInt,
                TypeKind::Integer(IntegerType {
                    signed: true,
                    bits: 0,
                    is_unspecified: true,
                }),
            ),
            (
                PrimitiveType::F32,
                TypeKind::Float(FloatType {
                    bits: 32,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::F64,
                TypeKind::Float(FloatType {
                    bits: 64,
                    is_unspecified: false,
                }),
            ),
            (
                PrimitiveType::UnspecifiedFloat,
                TypeKind::Float(FloatType {
                    bits: 0,
                    is_unspecified: true,
                }),
            ),
            (PrimitiveType::String, TypeKind::String),
            (PrimitiveType::Bool, TypeKind::Boolean),
            (PrimitiveType::Unknown, TypeKind::Unknown),
        ];

        for (ptype, kind) in types_to_register {
            self.register_primitive_type(ptype.name(), kind.clone(), TypeId(*ptype as usize));
        }
    }

    /// Registers a new type in the registry
    pub fn register_type(&mut self, name: &str, kind: TypeKind) -> TypeId {
        let id = TypeId::new();
        let type_info = TypeInfo {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };

        self.types.insert(id.clone(), type_info);

        id
    }

        /// Registers a new type in the registry
    pub fn register_primitive_type(&mut self, name: &str, kind: TypeKind, id: TypeId) -> TypeId {
        let type_info = TypeInfo {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };

        self.types.insert(id.clone(), type_info);

        id
    }

    /// Gets type information for a given TypeId
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to look up
    /// 
    /// ### Returns
    /// An Option containing the TypeInfo if found, or None if not found
    pub fn get_type_info(&self, id: &TypeId) -> Option<&TypeInfo> {
        self.types.get(id)
    }

    /// Try to get the primitive type for a given TypeId
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to look up
    /// 
    /// ### Returns
    /// An Option containing the PrimitiveType if found, or None if not found
    pub fn get_primitive_type(&self, id: &TypeId) -> Option<PrimitiveType> {
        self.get_type_info(id)
            .and_then(|info| PrimitiveType::from_str(&info.name))
    }

    /// Check if a type is a primitive type
    /// 
    /// ### Arguments
    /// * `id` - The TypeId to check
    /// 
    /// ### Returns
    /// A boolean indicating whether the type is a primitive type
    pub fn is_primitive_type(&self, id: &TypeId) -> bool {
        self.get_primitive_type(id).is_some()
    }

    /// Checks if a value is within the valid range for a given type
    /// 
    /// ### Arguments
    /// * `value` - The value to check
    /// * `type_id` - The TypeId of the type to check against
    /// 
    /// ### Returns
    /// A boolean indicating whether the value is within the valid range
    pub fn check_value_in_range(&self, value: &i64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };

        match &type_info.kind {
            TypeKind::Integer(int_type) => match (int_type.signed, int_type.bits) {
                (true, 32) => *value >= i32::MIN as i64 && *value <= i32::MAX as i64,
                (true, 64) => true,
                (false, 32) => *value >= 0 && *value <= u32::MAX as i64,
                (false, 64) => *value >= 0,
                _ => false,
            },
            TypeKind::Float(float_type) => match float_type.bits {
                32 => *value >= f32::MIN as i64 && *value <= f32::MAX as i64,
                64 => true,
                _ => *value >= f64::MIN as i64 && *value <= f64::MAX as i64,
            },
            _ => false,
        }
    }

    /// Checks if a float value is within the valid range for a given type
    /// 
    /// ### Arguments
    /// * `value` - The float value to check
    /// * `type_id` - The TypeId of the type to check against
    /// 
    /// ### Returns
    /// A boolean indicating whether the float value is within the valid range
    pub fn check_float_value_in_range(&self, value: &f64, type_id: &TypeId) -> bool {
        let type_info = match self.get_type_info(type_id) {
            Some(info) => info,
            None => return false,
        };

        match &type_info.kind {
            TypeKind::Float(float_type) => match float_type.bits {
                32 => *value >= f32::MIN as f64 && *value <= f32::MAX as f64,
                64 => true,
                _ => false,
            },
            _ => false,
        }
    }
}

