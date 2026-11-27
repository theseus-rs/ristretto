#![allow(dead_code)]
use ristretto_classloader::{ArrayElements, Object, Reference, Value};
use ristretto_gc::HeapBuffer;
enum Fields { Heap(HeapBuffer<Value>), Empty, One(Value), Two([Value;2]) }
enum FieldsWithDirty {
    Heap(HeapBuffer<Value>,Option<std::ops::Range<usize>>),
    Inline([Value;2], usize, Option<std::ops::Range<usize>>)
}
fn main() {
 println!("{{\"value\":{},\"heap_buffer\":{},\"candidate_storage\":{},\"array_elements\":{},\"object\":{},\"reference\":{},\"heap_buffer_with_dirty\":{},\"candidate_with_dirty\":{}}}",
 size_of::<Value>(), size_of::<HeapBuffer<Value>>(), size_of::<Fields>(),size_of::<ArrayElements>(),size_of::<Object>(),size_of::<Reference>(),size_of::<(HeapBuffer<Value>,Option<std::ops::Range<usize>>)>(),size_of::<FieldsWithDirty>());
}
