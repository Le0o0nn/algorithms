use rand::{Rng, rngs::ThreadRng};
use std::cmp::Ordering;

#[allow(dead_code)]
pub fn quick_sort(data: &mut [i32]) {
    quick_sort_2(data, &mut rand::rng());
}

fn quick_sort_2(data: &mut [i32], rng: &mut ThreadRng) {
    if data.len() < 2 {
        return;
    }
    let (lo, hi) = partition(data, data[rng.random_range(0..data.len())]);
    quick_sort_2(&mut data[..lo], rng);
    quick_sort_2(&mut data[hi..], rng);
}

fn partition(data: &mut [i32], pivot_val: i32) -> (usize, usize) {
    let (mut lo, mut hi) = (0, data.len());
    let mut i = 0;
    while i < hi {
        match data[i].cmp(&pivot_val) {
            Ordering::Less => {
                data.swap(i, lo);
                i += 1;
                lo += 1
            }
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Greater => {
                hi -= 1;
                data.swap(i, hi);
            }
        }
    }
    (lo, hi)
}

#[cfg(test)]
mod test {
    use super::quick_sort;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let mut data_1 = vec![rng.random_range(0..100); 1000];
            let mut data_2 = data_1.clone();
            quick_sort(&mut data_1);
            data_2.sort_unstable();
            assert_eq!(data_1, data_2);
        }
    }
}
