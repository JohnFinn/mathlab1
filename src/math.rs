extern crate statistical;

use std::ops::Deref;
use statistical::*;

pub fn intervals<T: Deref<Target=[f32]>>(collection: &T, interval: f32) -> Vec<i32> {
    let min = min(collection);
    let parts = ((max(collection) - min) / interval).ceil() as usize;
    let mut result = Vec::new();
    result.resize(parts, 0);
    for &elem in collection.iter() {
        let position = ((elem - min) / interval) as usize;
        result[position] += 1;
    }
    result
}


pub fn mode<T: Deref<Target=[f32]>>(collection: &T, interval: f32) -> f32 {
    let intervals = intervals(collection, interval);
    let index_of_max = intervals.iter().enumerate().max_by_key(|(_, &value)|value).unwrap().0;
    index_of_max as f32 * interval + min(collection)
}

pub fn min<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    let mut current = std::f32::MAX;
    for &x in collection.iter() {
        if x < current {
            current = x;
        }
    }
    current
}

pub fn max<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    let mut current = std::f32::MIN;
    for &x in collection.iter() {
        if x > current {
            current = x;
        }
    }
    current
}

pub fn standart_error<T: Deref<Target=[f32]>>(collection: &T) -> f32 {
    (variance(collection, None) / (collection.len() as f32)).sqrt()
}

pub fn probability_less_than<T: Deref<Target=[f32]>>(x: f32, collection: &T) -> f32 {
    let less_than_x_count = collection.iter().filter(|a| **a < x).count();
    less_than_x_count as f32 / collection.len() as f32
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intervals_test() {
        let a = vec![1.0, 1.1, 2.1, 3.2, 3.9];
        assert_eq!(intervals(&a, 1.0), vec![2, 1, 2]);
        let a = vec![1.0, 1.1, 3.0, 3.2];
        assert_eq!(intervals(&a, 1.0), vec![2, 0, 2]);
    }
}
