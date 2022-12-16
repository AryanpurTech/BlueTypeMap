use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

// TypeMap is our dependency "container" and provides us with ways to store and retrieve some value
// bound to its type.
//
// The `call` method allows dependencies to be automatically injected into a function with an argument
// length of up to twelve.
#[derive(Default)]
pub struct TypeMap {
    bindings: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {
    // bind stores the given value against its type within the container.
    pub fn bind<T: Any>(&mut self, val: T) {
        self.bindings.insert(val.type_id(), Box::new(val));
    }

    // get retrieves a reference to the value stored against the given type.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.bindings
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    // call calls the given callable with its arguments resolved from the values bound to their
    // types within the container.
    pub fn call<'a, F, Args>(&'a self, callable: F)
    where
        F: Callable<Args>,
        Args: FromTypeMap<'a>,
    {
        callable.call(Args::from_type_map(self));
    }
}

// FromTypeMap gives us a way to build some user defined type from the container.
pub trait FromTypeMap<'a> {
    fn from_type_map(type_map: &'a TypeMap) -> Self;
}

// Data is a container for a `T` that we can implement `FromTypeMap` on to allow a way for
// the users of our type-map to have any type resolved automatically into the callable.
pub struct Data<T: ?Sized>(Arc<T>);

impl<T> Data<T> {
    pub fn new(val: T) -> Self {
        Data(Arc::new(val))
    }
}

impl<T: ?Sized> Data<T> {
    pub fn get(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T: ?Sized> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data(self.0.clone())
    }
}

impl<T: ?Sized> Deref for Data<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Arc<T> {
        &self.0
    }
}

// This allows the Data<T> to be built from the container.
impl<T: ?Sized + 'static> FromTypeMap<'_> for Data<T> {
    fn from_type_map(type_map: &TypeMap) -> Self {
        type_map.get::<Self>().expect("type not found").clone()
    }
}

#[derive(Debug, Clone)]
pub struct DataMutStorage<T: ?Sized>(Arc<Mutex<T>>);

impl<T> DataMutStorage<T> {
    pub fn new(val: T) -> Self {
        Self(Arc::new(Mutex::new(val)))
    }
}

#[derive(Debug)]
pub struct DataMut<'a, T: ?Sized>(MutexGuard<'a, T>);

impl<T: ?Sized> Deref for DataMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: ?Sized> DerefMut for DataMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

impl<'a, T: ?Sized + 'static> FromTypeMap<'a> for DataMut<'a, T> {
    fn from_type_map(type_map: &'a TypeMap) -> Self {
        DataMut(
            type_map
                .get::<DataMutStorage<T>>()
                .expect("type not found")
                .0
                .lock()
                .unwrap(),
        )
    }
}

pub trait Callable<Args> {
    fn call(&self, args: Args);
}

// Here we implement `Callable` for tuples up to a length of twelve. Fn(A, B, C) for a tuple of three etc.
macro_rules! callable_tuple ({ $($param:ident)* } => {
    impl<Func, $($param,)*> Callable<($($param,)*)> for Func
    where
        Func: Fn($($param),*),
    {
        #[inline]
        #[allow(non_snake_case)]
        fn call(&self, ($($param,)*): ($($param,)*)) {
            (self)($($param,)*)
        }
    }
});

callable_tuple! {}
callable_tuple! { A }
callable_tuple! { A B }
callable_tuple! { A B C }
callable_tuple! { A B C D }
callable_tuple! { A B C D E }
callable_tuple! { A B C D E F }
callable_tuple! { A B C D E F G }
callable_tuple! { A B C D E F G H }
callable_tuple! { A B C D E F G H I }
callable_tuple! { A B C D E F G H I J }
callable_tuple! { A B C D E F G H I J K }
callable_tuple! { A B C D E F G H I J K L }
callable_tuple! { A B C D E F G H I J K L M}
callable_tuple! { A B C D E F G H I J K L M N}
callable_tuple! { A B C D E F G H I J K L M N O}
callable_tuple! { A B C D E F G H I J K L M N O P}
callable_tuple! { A B C D E F G H I J K L M N O P Q}
callable_tuple! { A B C D E F G H I J K L M N O P Q R}
callable_tuple! { A B C D E F G H I J K L M N O P Q R S}
callable_tuple! { A B C D E F G H I J K L M N O P Q R S T}

// Here we implement `FromTypeMap` for tuples up to a length of twelve. When `from_type_map` is
// called on the tuple it will return a new tuple with all of its arguments resolved from the container.
macro_rules! tuple_from_tm {
        ( $($T: ident )+ ) => {
            impl<'a, $($T: FromTypeMap<'a>),+> FromTypeMap<'a> for ($($T,)+)
            {
                #[inline]
                fn from_type_map(type_map: &'a TypeMap) -> Self {
                    ($($T::from_type_map(type_map),)+)
                }
            }
        };
    }

impl FromTypeMap<'_> for () {
    fn from_type_map(_type_map: &TypeMap) -> Self {
        ()
    }
}
tuple_from_tm! { A }
tuple_from_tm! { A B }
tuple_from_tm! { A B C }
tuple_from_tm! { A B C D }
tuple_from_tm! { A B C D E }
tuple_from_tm! { A B C D E F }
tuple_from_tm! { A B C D E F G }
tuple_from_tm! { A B C D E F G H }
tuple_from_tm! { A B C D E F G H I }
tuple_from_tm! { A B C D E F G H I J }
tuple_from_tm! { A B C D E F G H I J K }
tuple_from_tm! { A B C D E F G H I J K L }
tuple_from_tm! { A B C D E F G H I J K L M}
tuple_from_tm! { A B C D E F G H I J K L M N}
tuple_from_tm! { A B C D E F G H I J K L M N O}
tuple_from_tm! { A B C D E F G H I J K L M N O P}
tuple_from_tm! { A B C D E F G H I J K L M N O P Q}
tuple_from_tm! { A B C D E F G H I J K L M N O P Q R}
tuple_from_tm! { A B C D E F G H I J K L M N O P Q R S}
tuple_from_tm! { A B C D E F G H I J K L M N O P Q R S T}
