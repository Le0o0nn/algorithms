use std::cmp::Ordering;

#[allow(dead_code)]
pub fn merge_sort(data: &mut [i32]) {
    if data.len() < 2 {
        return;
    }
    let (data_lo, data_hi) = data.split_at_mut(data.len() / 2);
    merge_sort(data_lo);
    merge_sort(data_hi);
    merge(data_lo, data_hi);
}

fn merge(data_lo: &mut [i32], data_hi: &mut [i32]) {
    let mut temp = Vec::with_capacity(data_lo.len() + data_hi.len());
    let (mut i_lo, mut i_hi) = (0, 0);
    while i_lo < data_lo.len() && i_hi < data_hi.len() {
        match data_lo[i_lo].cmp(&data_hi[i_hi]) {
            Ordering::Less => {
                temp.push(data_lo[i_lo]);
                i_lo += 1;
            }
            _ => {
                temp.push(data_hi[i_hi]);
                i_hi += 1;
            }
        }
    }
    temp.extend_from_slice(&data_lo[i_lo..]);
    temp.extend_from_slice(&data_hi[i_hi..]);
    data_lo.clone_from_slice(&temp[..data_lo.len()]);
    data_hi.clone_from_slice(&temp[data_lo.len()..]);
}

#[cfg(test)]
mod test {
    use super::merge_sort;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let mut data_1 = vec![rng.random_range(0..100); 1000];
            let mut data_2 = data_1.clone();
            merge_sort(&mut data_1);
            data_2.sort_unstable();
            assert_eq!(data_1, data_2);
        }
    }
}
