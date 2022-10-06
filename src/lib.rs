//! # Ref is a Reference that is shared with multiples threads
//!
//! Equivalent `Arc<Mutex<T>>`
//!
//! `Arc<Mutex<T>>` is safer for things that need to be safe
//!
//! This will only be locked when is modified!
//!
//! This is good because you all ways can see the value.
//!
//! Is posibile to add some unexpected behaiviour!
//!
//! Is not very tested but for now is works!
//!
//! Use cases, when you want to make a game or a application,
//! the application has some buttons or objects that can be modificated from other thread,
//! but you always can see the value, for example for a renderer will not block the modifications from happening when is rendering,
//! but this means that the state will not be the most up to date.
//!
//! ### Attention is not really tested, you should use `Arc<Mutex<T>>` insted!
//!
//! If you know what you doing you can use this!
//!
//! Not warrenty if what you make, will not break!
//!
//! But open a issue on github if you find a bug/problem
mod r#ref;
mod ref_inner;
mod ref_mut;

pub use r#ref::Ref;
