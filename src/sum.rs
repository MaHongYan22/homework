pub fn total(list: &[u32]) -> Option<u32> {
    let mut x = list.iter();
    x.try_fold(0u32, |acc, &x| acc.checked_add(x))
}
