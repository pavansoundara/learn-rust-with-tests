fn array_sum(arr: [i32; 5]) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..arr.len() {
        sum += arr[i];
    }
    return sum;
}

fn slice_sum(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for v in slice {
        sum += v;
    }
    return sum;
}

fn all_sum(slices: &[&[i32]]) -> Vec<i32> {
    let mut res = vec![0; slices.len()];
    for i in 0..slices.len() {
        res[i] = slice_sum(slices[i]);
    }
    return res;
}

fn main() {
    println!("{}", array_sum([1, 2, 3, 4, 5]));
    println!("{}", slice_sum(&[1, 2, 3]));
    all_sum(&[&[1, 2]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_sum() {
        assert_eq!(array_sum([1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_slice_sum() {
        assert_eq!(slice_sum(&[1, 2, 3]), 6);
    }

    #[test]
    fn test_all_sum() {
        assert_eq!(all_sum(&[&[1, 2], &[2, 3]]), &[3, 5]);
    }
}
