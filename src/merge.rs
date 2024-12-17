use std::ops::AddAssign;

pub fn merge_dupes<T, Ti>(list: &mut Vec<(T, Ti)>)
where
    T: PartialEq<T> + Ord,
    Ti: AddAssign<Ti> + Copy,
{
    merge_dupes_with(list, |a, b| b.1 += a.1);
}
pub fn merge_dupes_with<T: PartialEq<T> + Ord, To>(
    list: &mut Vec<(T, To)>,
    merger: impl Fn(&mut (T, To), &mut (T, To)),
) {
    list.sort_by(|a, b| a.0.cmp(&b.0));
    list.dedup_by(|a, b| {
        if a.0 != b.0 {
            return false;
        }
        merger(a, b);
        true
    })
}
