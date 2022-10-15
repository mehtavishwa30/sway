use std::collections::HashMap;

use sway_types::{state::StateIndex, Ident, Span};

use crate::{
    language::{ty, typed::*, AsmOp, AsmRegister, CallPath, LazyOp, Literal},
    type_system::*,
};

pub(crate) struct TypedExpression {
    pub(crate) expression: TypedExpressionVariant,
    pub(crate) return_type: ResolvedType,
    pub(crate) span: Span,
}

pub(crate) enum TypedExpressionVariant {
    Literal(Literal),
    FunctionApplication {
        call_path: CallPath,
        contract_call_params: HashMap<String, TypedExpression>,
        arguments: Vec<(Ident, TypedExpression)>,
        function_decl: TypedFunctionDeclaration,
        self_state_idx: Option<StateIndex>,
        selector: Option<ty::ContractCallParams>,
    },
    LazyOperator {
        op: LazyOp,
        lhs: Box<TypedExpression>,
        rhs: Box<TypedExpression>,
    },
    VariableExpression {
        name: Ident,
        span: Span,
        mutability: ty::VariableMutability,
    },
    Tuple {
        fields: Vec<TypedExpression>,
    },
    Array {
        contents: Vec<TypedExpression>,
    },
    ArrayIndex {
        prefix: Box<TypedExpression>,
        index: Box<TypedExpression>,
    },
    StructExpression {
        struct_name: Ident,
        fields: Vec<TypedStructExpressionField>,
        span: Span,
    },
    CodeBlock(TypedCodeBlock),
    FunctionParameter,
    IfExp {
        condition: Box<TypedExpression>,
        then: Box<TypedExpression>,
        r#else: Option<Box<TypedExpression>>,
    },
    AsmExpression {
        registers: Vec<TypedAsmRegisterDeclaration>,
        body: Vec<AsmOp>,
        returns: Option<(AsmRegister, Span)>,
        whole_block_span: Span,
    },
    StructFieldAccess {
        prefix: Box<TypedExpression>,
        field_to_access: TypedStructField,
        field_instantiation_span: Span,
        resolved_prefix_type_id: TypeId,
    },
    TupleElemAccess {
        prefix: Box<TypedExpression>,
        index: usize,
        resolved_prefix_type_id: TypeId,
        index_span: Span,
    },
    EnumInstantiation {
        enum_decl: TypedEnumDeclaration,
        variant_name: Ident,
        tag: usize,
        contents: Option<Box<TypedExpression>>,
        enum_instantiation_span: Span,
        variant_instantiation_span: Span,
    },
    AbiCast {
        abi_name: CallPath,
        address: Box<TypedExpression>,
        #[allow(dead_code)]
        // this span may be used for errors in the future, although it is not right now.
        span: Span,
    },
    StorageAccess(TypedStorageAccess),
    IntrinsicFunction(TypedIntrinsicFunctionKind),
    AbiName(AbiName),
    EnumTag {
        exp: Box<TypedExpression>,
    },
    UnsafeDowncast {
        exp: Box<TypedExpression>,
        variant: TypedEnumVariant,
    },
    WhileLoop {
        condition: Box<TypedExpression>,
        body: TypedCodeBlock,
    },
    Break,
    Continue,
    Reassignment(Box<TypedReassignment>),
    StorageReassignment(Box<TypedStorageReassignment>),
    Return(Box<TypedExpression>),
}
