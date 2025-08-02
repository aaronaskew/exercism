#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Clone>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.is_empty() {
        return Comparison::Sublist;
    } else if second_list.is_empty() {
        return Comparison::Superlist;
    } else if second_list
        .windows(first_list.len())
        .any(|window| first_list == window)
    {
        return Comparison::Sublist;
    } else if first_list
        .windows(second_list.len())
        .any(|window| second_list == window)
    {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
