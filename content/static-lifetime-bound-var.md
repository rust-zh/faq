# `&'static T`、`T: 'static`、`static` 关键字分别表示什么？它们之间有什么联系？

## `&'static T`

`&'static T` 表示的是一个指向类型为 `T` 的值的引用，它的[生命周期]为 `'static`。这里的生命周期 `'static` 在语义上表示这个引用所指向的值在程序的整个运行期间都不会被释放（但可以使用 `unsafe` 构造违反这一语义的情况）。

这样的引用最常来自于[字面量]和对字面量的引用（如字符串字面量 `"hello world"` 的类型为 `&'static str`，`&[1i32, 2, 3]` 的类型为 `&'static [i32; 3]`），但它也可以来自于对常量或静态变量的引用，以及通过如 [`Vec::leak`][vec-leak] 等方法放弃[所有权]来获得。

由于 `'static` 生命周期长于任何其他生命周期，一个 `&'static T` 类型的引用可以安全地[强制转换](<> "coerce")为一个标注为任何生命周期的引用 `&'a T`。

## `T: 'static`

`T: 'static` 表示的是一个类型约束，它表明类型 `T` 可以在程序的整个运行期间有效，也即 `T` 中不包含引用，或包含的所有引用的生命周期都为 `'static`，举例来说：
* `struct Alpha(String, Vec<usize>);` 有 `Alpha: 'static`，因为类型中不包含引用；
* `struct Beta(&'static str);` 也有 `Beta: 'static`，因为类型中的所有引用都是 `'static` 的；
* `struct Gamma<'a>(&'a [usize]);` 仅当在 `'a` 为 `'static` 时有 `Gamma<'a>: 'static`，否则不满足；
* `struct Delta<T>(T);` 仅当 `T: 'static` 时有 `Delta<T>: 'static`，否则不满足；

如果一个值的类型不满足 `T: 'static`，则这个值必须在类型上所标注的生命周期结束之前被释放掉。如果满足，则这个值可以存在任意长时间。显然一个值的生命周期不能超越约束其类型的生命周期，因此任何 `&'static T` 引用中的 `T` 必然要满足 `T: 'static`。

## `static` 关键字

以 [`static` 关键字][static-kw]声明的，形如 `static FOO: [i32; 5] = [1, 2, 3, 4, 5];` 的变量是[静态变量]。

一个静态变量在程序的整个运行期间是唯一的，有唯一的地址，而且不会被释放。它可以被看作是 Rust 里的全局变量。显然如果一个静态变量的类型为 `T`，这个类型必须满足 `T: 'static`，如此一来，这个类型的引用才是 `&'static T`。


[vec-leak]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.leak
[static-kw]: https://doc.rust-lang.org/stable/std/keyword.static.html
