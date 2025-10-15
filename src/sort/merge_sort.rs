use std::{cmp::Ordering, mem};

pub fn merge_sort<T: Ord + Default>(data: &mut [T]) {
    let len = data.len();
    if len < 2 {
        return;
    }
    let (mut data1, mut data2) = data.split_at_mut(len / 2);
    merge_sort(&mut data1);
    merge_sort(&mut data2);
    merge(data1, data2);
}

fn merge<T: Ord + Default>(data1: &mut [T], data2: &mut [T]) {
    let (len1, len2) = (data1.len(), data2.len());
    let mut temp = Vec::with_capacity(len1 + len2);

    let (mut i1, mut i2) = (0, 0);
    while i1 < len1 && i2 < len2 {
        match data1[i1].cmp(&data2[i2]) {
            Ordering::Less => {
                temp.push(mem::take(&mut data1[i1]));
                i1 += 1;
            }
            _ => {
                temp.push(mem::take(&mut data2[i2]));
                i2 += 1;
            }
        }
    }
    data1[i1..].iter_mut().for_each(|e| temp.push(mem::take(e)));
    data2[i2..].iter_mut().for_each(|e| temp.push(mem::take(e)));

    temp[..len1]
        .iter_mut()
        .enumerate()
        .for_each(|(i, e)| data1[i] = mem::take(e));
    temp[len1..]
        .iter_mut()
        .enumerate()
        .for_each(|(i, e)| data2[i] = mem::take(e));
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    #[test]
    fn test1() {
        let mut data = Vec::<i32>::new();
        merge_sort(&mut data);
        assert_eq!(data, Vec::<i32>::new());
    }

    #[test]
    fn test2() {
        let mut data = vec![1];
        merge_sort(&mut data);
        assert_eq!(data, vec![1]);
    }

    #[test]
    fn test3() {
        let mut data = vec![2, 1];
        merge_sort(&mut data);
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test4() {
        let mut rng = rand::rng();
        for _ in 1..=100 {
            let mut data1 = vec![rng.random::<i32>(), 1000];
            let mut data2 = data1.clone();
            merge_sort(&mut data1);
            data2.sort_unstable();
            assert_eq!(data1, data2);
        }
    }
}
