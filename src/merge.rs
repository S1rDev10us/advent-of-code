use std::ops::AddAssign;

pub fn merge_dupes<T, Ti>(list: &mut Vec<(T, Ti)>)
where
    T: PartialEq<T> + Ord,
    Ti: AddAssign<Ti> + Copy,
{
    list.sort_by(|a, b| a.0.cmp(&b.0));
    list.dedup_by(|a, b| {
        if a.0 != b.0 {
            return false;
        }
        b.1 += a.1;
        true
    })
}
