# 如何在特质里添加异步函数？

目前 Rust 不支持在[特质]里直接添加[异步函数]，但可以使用 [async-trait] 这个库来实现。这个库会将异步函数改写为返回 `Pin<Box<dyn Future>>` 的普通函数以绕过目前语言层面的限制，但也因此有堆分配以及[动态分发]这两个额外的代价，所以不会被直接添加到 Rust 语言中。

在特质里不支持使用异步函数是由于异步函数本质上是一个返回 `impl Future<Output = T>` 的函数，而目前 Rust 的类型系统还无法表达在特质的方法的返回类型上使用 `impl Trait`。有两个已经通过的 RFC 旨在解决这一问题：[RFC 1598][rfc1598] 泛型关联类型和 [RFC 2071][rfc2071] `impl Trait` [存在类型]，但它们的编译器支持还在实现中，实现进度可以参考 [impl Trait 计划][impl-trait-initiative]的页面。


[async-trait]: https://crates.io/crates/async-trait

[rfc1598]: https://rust-lang.github.io/rfcs/1598-generic_associated_types.html
[rfc2071]: https://rust-lang.github.io/rfcs/2071-impl-trait-existential-types.html
[impl-trait-initiative]: https://rust-lang.github.io/impl-trait-initiative/
