# 如何直接在堆上分配新的对象或数组？

目前 Rust 语言本身没有提供稳定且不使用 `unsafe` 的方式能保证将一个对象或数组直接分配到堆上。

`Box::new([0; 4096])` 等方式在语义上是在栈上创建数组，然后再移动到堆上。[`Vec`][vec] 等容器类型的内容会直接分配在堆上，因而也可以通过 [`Vec::into_boxed_slice`][into-boxed-slice] 从一个 `Vec` 得到堆上切片 `Box<[T]>`，再通过 [`TryFrom`][try-from-boxed-slice] 获得堆上数组 `Box<[T; N]>`。不过添加每个元素从语义上依然是在栈上分配再移入容器的。

有一些第三方的库，如 [copyless]、[boxext] 和 [default-boxed] 等，通过依赖编译器优化或包装 `unsafe` 的功能来提供安全的接口进行直接分配。

使用 `unsafe` 的话可以通过调用 [`alloc`][alloc] 函数直接分配堆内存并取得指针，但需要手动初始化和管理分配的内存。`Box` 及其他智能指针类型未来很可能会提供 [`new_uninit`][box-new-uninit]、[`new_uninit_slice`][box-new-uninit-slice] 等方法在堆上直接创建 [`MaybeUninit`][maybe-uninit]，但你仍将需要使用 `unsafe` 的方式来初始化其内容。

未稳定的 [`box` 语法][box-syntax]在一些情况下可以直接分配到堆上并创建一个 `Box`，但当有嵌套表达式，如 `box Wrapper([0; 4096])`，时则依然会有先分配在栈上再移入堆的问题。而且 `box` 语法目前也没有稳定化的计划。

此外，有一些提案，如 [RFC 2884][rfc-2884]，试图提供新的接口来解决这一问题，但目前还没有足够的共识。


[vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[into-boxed-slice]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_boxed_slice
[try-from-boxed-slice]: https://doc.rust-lang.org/std/boxed/struct.Box.html#impl-TryFrom%3CBox%3C%5BT%5D%3E%3E
[alloc]: https://doc.rust-lang.org/std/alloc/fn.alloc.html
[box-new-uninit]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_uninit
[box-new-uninit-slice]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_uninit_slice
[maybe-uninit]: https://doc.rust-lang.org/std/mem/union.MaybeUninit.html

[box-syntax]: https://doc.rust-lang.org/nightly/unstable-book/language-features/box-syntax.html
[rfc-2884]: https://github.com/rust-lang/rfcs/pull/2884

[copyless]: https://crates.io/crates/copyless
[boxext]: https://crates.io/crates/boxext
[default-boxed]: https://crates.io/crates/default-boxed
