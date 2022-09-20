# 错误处理推荐使用什么库？

目前一般认为对于应用程序推荐使用 [anyhow]，而对于库推荐使用 [thiserror]。

anyhow 提供了一个基于[特质对象]的错误类型，可以很容易地将不同来源的错误统一到单一类型，并可以方便地为错误添加上下文，以及就地创建新的错误。

thiserror 则提供了一个 derive 宏，方便为自定义的错误类型实现 [`Error` 特质][error-trait]。


[anyhow]: https://crates.io/crates/anyhow
[thiserror]: https://crates.io/crates/thiserror
[error-trait]: https://doc.rust-lang.org/std/error/trait.Error.html
