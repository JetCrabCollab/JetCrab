use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConstantIndex(usize);

impl ConstantIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ConstantIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<ConstantIndex> for usize {
    fn from(idx: ConstantIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GlobalIndex(usize);

impl GlobalIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for GlobalIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<GlobalIndex> for usize {
    fn from(idx: GlobalIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalIndex(usize);

impl LocalIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for LocalIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<LocalIndex> for usize {
    fn from(idx: LocalIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArgIndex(usize);

impl ArgIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ArgIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<ArgIndex> for usize {
    fn from(idx: ArgIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FunctionIndex(usize);

impl FunctionIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for FunctionIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<FunctionIndex> for usize {
    fn from(idx: FunctionIndex) -> Self {
        idx.0
    }
}

impl fmt::Display for FunctionIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArraySize(usize);

impl ArraySize {
    pub fn new(size: usize) -> Self {
        Self(size)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for ArraySize {
    fn from(size: usize) -> Self {
        Self(size)
    }
}

impl From<ArraySize> for usize {
    fn from(size: ArraySize) -> Self {
        size.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StackIndex(usize);

impl StackIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}

impl From<usize> for StackIndex {
    fn from(index: usize) -> Self {
        Self(index)
    }
}

impl From<StackIndex> for usize {
    fn from(idx: StackIndex) -> Self {
        idx.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FramePointer(usize);

impl FramePointer {
    pub fn new(pointer: usize) -> Self {
        Self(pointer)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for FramePointer {
    fn from(pointer: usize) -> Self {
        Self(pointer)
    }
}

impl From<FramePointer> for usize {
    fn from(ptr: FramePointer) -> Self {
        ptr.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectId(usize);

impl ObjectId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }
}

impl From<usize> for ObjectId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<ObjectId> for usize {
    fn from(id: ObjectId) -> Self {
        id.0
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapId(usize);

impl HeapId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.0 != usize::MAX
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<usize> for HeapId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl From<HeapId> for usize {
    fn from(id: HeapId) -> Self {
        id.0
    }
}

impl fmt::Display for HeapId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Heap({})", self.0)
    }
}
