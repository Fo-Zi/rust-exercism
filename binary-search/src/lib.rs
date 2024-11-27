pub fn find<T: Eq + Ord>(array: &[T], key: T) -> Option<usize> {
    let mut index_low = 0;
    let mut index_high = array.len();

    while index_low < index_high {
        let mid = index_low + (index_high - index_low) / 2;

        match &array[mid] {
            elem if *elem == key => return Some(mid),
            elem if *elem < key => index_low = mid + 1,
            _ => index_high = mid,
        }
    }

    None
}
