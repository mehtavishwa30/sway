use crate::language::typed::*;

pub(crate) enum TypedDeclaration {
    VariableDeclaration(Box<TypedVariableDeclaration>),
    ConstantDeclaration(TypedConstantDeclaration),
    FunctionDeclaration(TypedFunctionDeclaration),
    TraitDeclaration(TypedTraitDeclaration),
    StructDeclaration(TypedStructDeclaration),
    EnumDeclaration(TypedEnumDeclaration),
    ImplTrait(TypedImplTrait),
    AbiDeclaration(TypedAbiDeclaration),
    StorageDeclaration(TypedStorageDeclaration),
    ErrorRecovery,
}
