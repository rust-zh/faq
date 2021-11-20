# 为什么使用调试模式构建的 Rust 程序运行速度很慢？

当不指定任何标志时使用 `cargo build` 或 `cargo run` 来构建或运行 Rust 程序，会默认使用调试模式来进行构建，这时编译出的 Rust 程序运行速度很慢，有时甚至不及等效的 Python 代码。加上 `--release` 标志时会使用发布模式来构建，这样编译出的程序运行速度会快得多。造成这种现象的原因主要有两点：

一是调试模式构建主要是针对编译速度而非运行速度进行优化，这是为了减短编辑-编译-调试这个循环所需的时间。当为编译速度优化时，一方面，很多为加快运行速度但会拖慢编译的编译器优化会被禁用。由于 Rust 的库和程序通常有很多抽象设施，它们虽然给编写程序带来便利，但在调试模式下却可能无法被完全优化掉，进而拖慢运行速度。另一方面，调试模式默认下会启用一些额外的机制加快编译速度，如[增量编译](<> "incremental compilation")和[并发代码生成](<> "parallel codegen")，这些优化虽然加快了编译速度，但同时也牺牲了编译结果的质量，降低了运行速度。

二是调试模式下会启用一些额外的检测，而这些检测会增加运行时间。在调试模式下，编译器默认会为内置整数类型的算数运算插入溢出检查，在溢出发生时 panic 以帮助发现潜在的逻辑问题，而在发布模式下则没有这一检查。此外还有提供如 [`debug_assert!`][debug_assert] 宏和 [`debug_assertions`][debug_assertions] 条件编译选项等设施让库和应用可以根据需要在调试模式下额外执行一些较为昂贵的运行时检查。


[debug_assert]: https://doc.rust-lang.org/std/macro.debug_assert.html
[debug_assertions]: https://doc.rust-lang.org/reference/conditional-compilation.html#debug_assertions
