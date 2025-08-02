pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    if &key < array.first().unwrap() {
        return None;
    }

    let mut l = 0_usize;
    let mut r = array.len() - 1;

    while l <= r {
        let m = (l + r) / 2;
        dbg!(m);
        match array[m].cmp(&key) {
            std::cmp::Ordering::Less => {
                l = m + 1;
            }
            std::cmp::Ordering::Greater => {
                r = m - 1;
            }
            std::cmp::Ordering::Equal => {
                return Some(m);
            }
        }
    }
    None
}
