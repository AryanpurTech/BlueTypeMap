# Blue TypeMap

A TypeMap system that is copied from [here](https://nickbryan.co.uk/software/using-a-type-map-for-dependency-injection-in-rust/) that can be used to create dynamic function paramters for cleaner APIs.

It is similar to what [Bevy](https://github.com/bevyengine/bevy) and [Actix Web](https://github.com/actix/actix-web) uses, and now you can too in your projects. The reason I made it into a crate is because of frustration when learning about the topic, so that other's won't have to go through it. **I highly advice read the blog above for more information**.

## Example usage

```rust
use blue_typemap::mutable::{Data, TypeMap, Data};

fn function1() {
    println!("no parameter");
}
fn function2(data: Data<u32>) {
    // be sure to use .get() to obtain the value
    let data = data.get();
    println!("an int {}", data);
}

fn function3(data: Data<&str>) {
    println!("a str {}", data.get());
}

fn function4(mut data: DataMut<i32>) {
    // they're sent as mutexguards and have deref, so can easily change values of,
    // thanks to help from bruh![moment] at Rust Programming Language discord server on thread
    // `Typemap dependency injection mutation`.
    let mut x = data;
    *x *= 2;
}

fn main() {
    // Create a TypeMap container
    let mut container = TypeMap::default();
    // Bind values to it
    container.bind(Data::new("Values"));
    container.bind(Data::new(vec!["name", "age", "birth"]));
    container.bind(Data::new(50u32));
    // Mutable values
    container.bind(DataMutStorage::new(10i32));
    // Call the functions
    container.call(function1);
    container.call(function2);
    container.call(function3);
    container.call(function4);
    // Works with closures too
    container.call(|fields: Data<Vec<&str>>| {
        for i in fields.get() {
            println!("Field: {}", i);
        }
    });
}
```

## Extra details

Currently it's very very much slower, obviously, than normal functions and function calls. the overhead is significant that make it undesirable on programs that performance is critical. Hence, use it only when performance isn't required as much as you can get.

Also currently around 20 parameters are supported. You can increase it by running `callable_tuple!` and `tuple_from_tm!` macros, continuing after what they've been written as in the `src/lib.rs`. Not all types are supported so you might want to implement `FromTypeMap<'a>` for your types.

Have a nice journey ahead!
