# 如果有一个 `trait Foo: Base`，如何将一个 `&dyn Foo` 转换到 `&dyn Base`？

Rust 目前不直接提供这种转换，如果需要转换可以使用一个中间特质来实现，如
```rust
trait Base {
    // ...
}

trait AsBase {
    fn as_base(&self) -> &dyn Base;
}

impl<T: Base> AsBase for T {
    fn as_base(&self) -> &dyn Base { self }
}

trait Foo: AsBase {
    // ...
}
```

不支持的主要原因是在特质对象的虚表中没有相应的数据指向另一个特质的虚表，而不提供相应数据的原因可能是由于这很容易产生过多无用的虚表，进而导致二进制体积的膨胀。

更多关于这一话题的讨论可以参考 [RFC 2765][rfc2765] 以及 [Traits, dynamic dispatch and upcasting][bchlr]。


[rfc2765]: https://github.com/rust-lang/rfcs/issues/2765
[bchlr]: https://articles.bchlr.de/traits-dynamic-dispatch-upcasting
