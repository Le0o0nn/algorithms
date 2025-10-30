use rand::Rng;
use std::cmp::Ordering;

#[allow(dead_code)]
pub fn get_k_min(data: &[i32], k: usize) -> Option<i32> {
    if k > data.len() {
        return None;
    }
    let mut temp = data.to_owned();
    let mut data = temp.as_mut_slice();
    let mut k = k - 1;
    let mut rng = rand::rng();
    loop {
        let (lo, hi) = partition(data, data[rng.random_range(0..data.len())]);
        if k < lo {
            data = &mut data[..lo];
            continue;
        }
        if k >= hi {
            data = &mut data[hi..];
            k -= hi;
            continue;
        }
        break Some(data[k]);
    }
}

fn partition(data: &mut [i32], pivot_val: i32) -> (usize, usize) {
    let (mut lo, mut hi) = (0, data.len());
    let mut i = 0;
    while i < hi {
        match data[i].cmp(&pivot_val) {
            Ordering::Less => {
                data.swap(i, lo);
                i += 1;
                lo += 1;
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
    use super::get_k_min;
    use rand::seq::SliceRandom;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        let mut data: Vec<i32> = (1..=1000).collect();
        data.shuffle(&mut rng);
        for k in 1..=1000 {
            assert_eq!(get_k_min(&data, k), Some(k as i32));
        }
    }
}
