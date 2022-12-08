//! Each of the valid `Value` types.
//!
//! These generally mimic the Sway types with a couple of exceptions:
//! - [`Type::Unit`] is still a discrete type rather than an empty tuple.  This may change in the
//!   future.
//! - [`Type::Union`] is a sum type which resembles a C union.  Each member of the union uses the
//!   same storage and the size of the union is the size of the largest member.
//!
//! [`Aggregate`] is an abstract collection of [`Type`]s used for structs, unions and arrays,
//! though see below for future improvements around splitting arrays into a different construct.

use std::{cell::Ref, collections::hash_map::Entry};

use generational_arena::Arena;

use crate::{context::Context, pretty::DebugWithContext, Constant, ConstantValue, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, DebugWithContext)]
pub struct Type(pub generational_arena::Index);

#[derive(Debug, Clone, DebugWithContext, Hash, PartialEq, Eq)]
pub enum TypeContent {
    Unit,
    Bool,
    Uint(u8),
    B256,
    String(u64),
    Array(Type, u64),
    Union(Vec<Type>),
    Struct(Vec<Type>),
    Pointer(Type),
}

impl Type {
    // TODO: We could cache these in Context upon its creation for unit, bool etc.
    fn get_or_create_unique_type(context: &Context, t: TypeContent) -> Type {
        let mut entry = context.type_map.borrow_mut();
        let entry = entry.entry(t.clone());
        match entry {
            Entry::Occupied(oe) => *oe.get(),
            Entry::Vacant(ve) => {
                let new_entry = Type(context.types.borrow_mut().insert(t));
                ve.insert(new_entry);
                new_entry
            }
        }
    }

    pub fn get_content<'a>(&self, context: &'a Context) -> Ref<'a, TypeContent> {
        Ref::<Arena<TypeContent>>::map(context.types.borrow(), |t| &t[self.0])
    }

    /// Get unit type
    pub fn get_unit(context: &Context) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Unit)
    }

    /// Get bool type
    pub fn get_bool(context: &Context) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Bool)
    }

    /// Get unsigned integer type
    pub fn get_uint(context: &Context, width: u8) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Uint(width))
    }

    /// Get B256 type
    pub fn get_b256(context: &Context) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::B256)
    }

    /// Get string type
    pub fn get_string(context: &Context, len: u64) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::String(len))
    }

    /// Get array type
    pub fn get_array(context: &Context, elm_ty: Type, len: u64) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Array(elm_ty, len))
    }

    /// Get union type
    pub fn get_union(context: &Context, fields: Vec<Type>) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Union(fields))
    }

    /// Get struct type
    pub fn get_struct(context: &Context, fields: Vec<Type>) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Struct(fields))
    }

    /// Get pointer type
    pub fn get_pointer(context: &Context, pointee_ty: Type) -> Type {
        Self::get_or_create_unique_type(context, TypeContent::Pointer(pointee_ty))
    }

    /// Return whether this is a 'copy' type, one whose value will always fit in a register.
    pub fn is_copy_type(&self, context: &Context) -> bool {
        matches!(
            *self.get_content(context),
            TypeContent::Unit
                | TypeContent::Bool
                | TypeContent::Uint(_)
                | TypeContent::Pointer { .. }
        )
    }

    /// Return a string representation of type, used for printing.
    pub fn as_string(&self, context: &Context) -> String {
        let sep_types_str = |agg_content: &Vec<Type>, sep: &str| {
            agg_content
                .iter()
                .map(|ty| ty.as_string(context))
                .collect::<Vec<_>>()
                .join(sep)
        };

        match &*self.get_content(context) {
            TypeContent::Unit => "()".into(),
            TypeContent::Bool => "bool".into(),
            TypeContent::Uint(nbits) => format!("u{}", nbits),
            TypeContent::B256 => "b256".into(),
            TypeContent::String(n) => format!("string<{}>", n),
            TypeContent::Array(ty, cnt) => {
                format!("[{}; {}]", ty.as_string(context), cnt)
            }
            TypeContent::Union(agg) => {
                format!("( {} )", sep_types_str(&agg, " | "))
            }
            TypeContent::Struct(agg) => {
                format!("{{ {} }}", sep_types_str(&agg, ", "))
            }
            TypeContent::Pointer(pointee_ty) => {
                format!("ptr {}", pointee_ty.as_string(context))
            }
        }
    }

    /// Compare a type to this one for equivalence.
    /// `PartialEq` does not take into account the special case for Unions below.
    pub fn eq(&self, context: &Context, other: &Type) -> bool {
        match (&*self.get_content(context), &*other.get_content(context)) {
            (TypeContent::Unit, TypeContent::Unit) => true,
            (TypeContent::Bool, TypeContent::Bool) => true,
            (TypeContent::Uint(l), TypeContent::Uint(r)) => l == r,
            (TypeContent::B256, TypeContent::B256) => true,
            (TypeContent::String(l), TypeContent::String(r)) => l == r,

            (TypeContent::Array(l, llen), TypeContent::Array(r, rlen)) => {
                llen == rlen && l.eq(context, &r)
            }
            (TypeContent::Struct(l), TypeContent::Struct(r))
            | (TypeContent::Union(l), TypeContent::Union(r)) => {
                l.len() == r.len() && l.iter().zip(r.iter()).all(|(l, r)| l.eq(context, r))
            }
            // Unions are special.  We say unions are equivalent to any of their variant types.
            (_, TypeContent::Union(_)) => other.eq(context, self),
            (TypeContent::Union(l), _) => l.iter().any(|field_ty| other.eq(context, field_ty)),

            (TypeContent::Pointer(l), TypeContent::Pointer(r)) => l.eq(context, &r),
            _ => false,
        }
    }

    /// Gets the inner pointer type if its a pointer.
    pub fn get_inner_ptr_type(&self, context: &Context) -> Option<Type> {
        match *self.get_content(context) {
            TypeContent::Pointer(pointee_typ) => Some(pointee_typ),
            _ => None,
        }
    }

    /// If this type is a pointer then return the pointed to type, else return self.
    pub fn strip_ptr_type(&self, context: &Context) -> Type {
        self.get_inner_ptr_type(context).unwrap_or(*self)
    }

    /// Is unit type
    pub fn is_unit(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Unit)
    }

    /// Is bool type
    pub fn is_bool(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Bool)
    }

    /// Is unsigned integer type
    pub fn is_uint(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Uint(_))
    }

    /// Is unsigned integer type of specific width
    pub fn is_uint_of(&self, context: &Context, width: u8) -> bool {
        matches!(*self.get_content(context), TypeContent::Uint(width_) if width == width_)
    }

    /// Is B256 type
    pub fn is_b256(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::B256)
    }

    /// Is string type
    pub fn is_string(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::String(_))
    }

    /// Is array type
    pub fn is_array(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Array(..))
    }

    /// Is union type
    pub fn is_union(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Union(_))
    }

    /// Is struct type
    pub fn is_struct(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Struct(_))
    }

    /// Returns true if this is a pointer type.
    pub fn is_ptr_type(&self, context: &Context) -> bool {
        matches!(*self.get_content(context), TypeContent::Pointer(_))
    }

    /// What's the type of the value indexed by a GEP.
    pub fn get_indexed_type(&self, context: &Context, indices: &[Value]) -> Option<Type> {
        if indices.is_empty() {
            return Some(*self);
        }

        // The first index is the offset (see LLVM's GEP, for example) from the pointer. Skip.
        // See: https://llvm.org/docs/GetElementPtr.html
        indices
            .iter()
            .skip(1)
            .fold(Some(self.strip_ptr_type(context)), |ty, idx| {
                ty.and_then(|ty| {
                    ty.get_field_type(context, *idx)
                        .or_else(|| ty.get_array_elem_type(context))
                })
            })
    }

    pub fn get_field_type(&self, context: &Context, idx: Value) -> Option<Type> {
        match (&*self.get_content(context), idx.get_constant(context)) {
            (
                TypeContent::Struct(agg),
                Some(Constant {
                    ty,
                    value: ConstantValue::Uint(idx),
                }),
            )
            | (
                TypeContent::Union(agg),
                Some(Constant {
                    ty,
                    value: ConstantValue::Uint(idx),
                }),
            ) if ty.is_uint_of(context, 64) => agg.get(*idx as usize).cloned(),
            // Trying to index a non-aggregate.
            _otherwise => None,
        }
    }

    /// Get the type of the array element, if applicable.
    pub fn get_array_elem_type(&self, context: &Context) -> Option<Type> {
        if let TypeContent::Array(ty, _) = *self.get_content(context) {
            Some(ty)
        } else {
            None
        }
    }

    /// Get the length of the array , if applicable.
    pub fn get_array_len(&self, context: &Context) -> Option<u64> {
        if let TypeContent::Array(_, n) = *self.get_content(context) {
            Some(n)
        } else {
            None
        }
    }

    pub fn is_aggregate(&self, context: &Context) -> bool {
        match *self.get_content(context) {
            TypeContent::Union(_) | TypeContent::Struct(_) | TypeContent::Array(..) => true,
            TypeContent::Unit
            | TypeContent::Bool
            | TypeContent::Uint(_)
            | TypeContent::B256
            | TypeContent::String(_)
            | TypeContent::Pointer(_) => false,
        }
    }
}
