extern crate statistical;

use std::ops::Deref;
use statistical::*;

pub fn standart_error<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    (variance(collection, None) / (collection.len() as f32)).sqrt()
}

pub fn probability_less_than<T: Deref<Target=[f32]>>(x: f32, collection: &T) -> f32 {
    let less_than_x_count = collection.iter().filter(|a| **a < x).count();
    less_than_x_count as f32 / collection.len() as f32
}
