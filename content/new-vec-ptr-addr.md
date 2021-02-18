# 为什么 `Vec::<T>::new().as_ptr()` 的值不是 `0`？

[`Vec<T>`](Vec) 内部使用的指针类型为 [`core::ptr::Unique<T>`](Unique)，它允许[悬垂](<> "dangling")但不允许零值。当空 `Vec<T>` 初始化时，没有内存被分配，此时内部指针是一个悬垂但[良好对齐](<> "well-aligned")的值。

这种指针表达了“拥有 `T` 的所有权”的语义，也能使编译器对一些包含 `Vec<T>` 的类型进行[空指针优化](https://doc.rust-lang.org/nomicon/repr-rust.html "null value optimization")。

[Vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[Unique]: https://github.com/rust-lang/rust/blob/master/library/core/src/ptr/unique.rs
