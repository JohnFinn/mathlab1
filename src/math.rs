use std::ops::Deref;

pub fn mean<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    let sum: f32 = collection.iter().sum();
    sum / collection.len() as f32
}

pub fn variance<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    let mean_value = mean(collection);
    let s: f32 = collection.iter().map(|x| (x - mean_value).powf(2.0)).sum();
    s / collection.len() as f32
}


pub fn standart_error<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    (variance(collection) / (collection.len() as f32)).sqrt()
}

pub fn probability_less_than<T: Deref<Target=[f32]>>(x: f32, collection: &T) -> f32 {
    let less_than_x_count = collection.iter().filter(|a| **a < x).count();
    less_than_x_count as f32 / collection.len() as f32
}
