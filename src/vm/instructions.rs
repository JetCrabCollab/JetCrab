use crate::vm::types::{
    ArgIndex, ArraySize, CodeAddress, ConstantIndex, FunctionIndex, GlobalIndex, LocalIndex,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    PushConst(ConstantIndex),
    Pop,
    Dup,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    Inc,
    Dec,

    And,
    Or,
    Not,
    Xor,

    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    StrictEq,
    StrictNe,

    LoadGlobal(GlobalIndex),
    StoreGlobal(GlobalIndex),
    LoadLocal(LocalIndex),
    StoreLocal(LocalIndex),
    LoadArg(ArgIndex),
    LoadThisFunction,
    LoadThis,
    LoadClosureVar(String),

    Jump(CodeAddress),
    JumpIfTrue(CodeAddress),
    JumpIfFalse(CodeAddress),

    Call(FunctionIndex),
    Return,

    NewObject,
    NewArray(ArraySize),
    SetProperty,
    GetProperty,

    TypeOf,
    InstanceOf,
    In,
    Delete,
    New,

    NewClass,
    GetPrototype,
    SetPrototype,

    Await,
    Yield,

    Throw,
    Try(CodeAddress, CodeAddress),
    Catch,
    Finally,

    Spread,
    Destructure,
    OptionalChain,
    NullishCoalesce,

    PushNull,
    PushUndefined,
    PushTrue,
    PushFalse,
    PushSymbol(ConstantIndex),
    PushBigInt(ConstantIndex),
    CallFunction(FunctionIndex, ArgIndex),
    RemoveObjectProperty,
    CallObjectMethod(String, ArgIndex),
    CallArrayMethod(String, ArgIndex),
    GetArrayLength,
    RemoveArrayElement(ArraySize),
    PushArrayElement,
    PopArrayElement,
    ShiftArrayElement,
    UnshiftArrayElement(ArraySize),
    SliceArray(ArraySize, ArraySize),
    ConcatArray(ArraySize),
    IndexOfArray(ArraySize),
    IncludesArray(ArraySize),

    Halt,
}
