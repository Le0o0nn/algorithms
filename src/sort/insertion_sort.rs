pub fn insertion_sort<T: Ord>(data: &mut [T]) {
    let len = data.len();
    if len < 2 {
        return;
    }
    for i in 1..len {
        let mut j = i;
        while j > 0 && data[j] < data[j - 1] {
            data.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use rand::Rng;
    use super::*;

    #[test]
    fn test1() {
        let mut data = Vec::<i32>::new();
        insertion_sort(&mut data);
        assert_eq!(data, Vec::<i32>::new());
    }

    #[test]
    fn test2() {
        let mut data = vec![1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1]);
    }

    #[test]
    fn test3() {
        let mut data = vec![2, 1];
        insertion_sort(&mut data);
        assert_eq!(data, vec![1, 2]);
    }

    #[test]
    fn test4() {
        let mut rng = rand::rng();
        for _ in 1..=100 {
            let mut data1 = vec![rng.random::<i32>(); 100];
            let mut data2 = data1.clone();
            insertion_sort(&mut data1);
            data2.sort_unstable();
            assert_eq!(data1, data2);
        }
    }
}