#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let sub = _first_list.is_empty() || _second_list.windows(_first_list.len()).any(|v| v == _first_list);
    let sup = _second_list.is_empty() || _first_list.windows(_second_list.len()).any(|v| v == _second_list);
    match (sub, sup) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal
    }
}
