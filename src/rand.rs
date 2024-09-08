use rand;
use std::ops;
use std::cmp;

pub fn rand_range<T>(max: T) -> T
where rand::distributions::Standard: rand::distributions::Distribution<T>,
T: ops::Rem<Output = T> {
    rand::random::<T>() % max
}

pub fn rand_roll<T>(odds: T) -> bool
where rand::distributions::Standard: rand::distributions::Distribution<T>,
T: ops::Rem<Output = T> + cmp::PartialOrd<u64> {
    rand_range(odds) > 1 
}

pub fn rand_elem<T>(arr: &[T]) -> &T 
where [T]: Sized {
    &arr[rand_range(arr.len())]
}

pub fn rand_shuffle<T>(arr: &mut [T]) {
    let n = arr.len();
    for i in (1..n).rev() {
        arr.swap(i, rand_range(i + 1));
    }
}