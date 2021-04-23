# 如何约束一个泛型参数为基本数值类型？

可以使用由 [num-traits] 所提供的 [`PrimInt`][prim-int] 和 [`Float`][float] 两个[特质]来约束泛型参数为基本整数类型和浮点数类型。


[num-traits]: https://crates.io/crates/num-traits
[prim-int]: https://docs.rs/num-traits/0.2.14/num_traits/int/trait.PrimInt.html
[float]: https://docs.rs/num-traits/0.2.14/num_traits/float/trait.Float.html
