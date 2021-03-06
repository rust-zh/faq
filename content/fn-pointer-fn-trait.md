# `fn()` 类型与 `Fn()` 等特质的关系和区别是什么？

在 Rust 中，每一个函数，无论是由 `fn` 关键字定义的一般函数，还是由闭包表达式定义的闭包，都有一个各自独立的匿名类型。为了能间接地使用函数，Rust 准备了两种方式，即 [`fn()`][fn] 类型与 [`Fn()`][Fn-trait]、[`FnMut()`][FnMut-trait] 和 [`FnOnce()`][FnOnce-trait] 等[特质]。

要表达不同的类型，最常见的方法即是使用特质（作为类型约束，即 `T: Fn()` 和 `impl Fn()`，或者使用[特质对象]，即 `dyn Fn()`），`Fn()` 一族就是用于表达函数类型的特质。

`fn()` 不是一个特质，而是一个具体的类型，表示一个函数指针。功能上它与特质对象类似，可以近似地看作 `&'static dyn Fn()`。但 `fn()` 与 `Fn()` 不同，它不包含对上下文的引用，因而只有一般函数或没有捕获任何上下文的闭包能够被转换成 `fn()`。因此它也与 `&dyn Fn()` 不同，不需要使用[胖指针]。它的大小与普通的指针一致。

因为 `fn()` 是一个函数指针，通过它调用函数与通过特质对象一样是间接调用，而使用 `Fn()` 等特质约束的泛型则是通过[单态化]来直接调用的。


[fn]: https://doc.rust-lang.org/std/primitive.fn.html
[Fn-trait]: https://doc.rust-lang.org/std/ops/trait.Fn.html
[FnMut-trait]: https://doc.rust-lang.org/std/ops/trait.FnMut.html
[FnOnce-trait]: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
