pub enum Order {
    Ascending,
    Descending,
}

pub fn sort<T: Ord>(arr: &mut [T], order: Order) {
    // Custom sorting using sort_by
    match order {
        Order::Ascending => arr.sort_by(|a, b| a.cmp(b)),
        Order::Descending => arr.sort_by(|a, b| b.cmp(a)),
    }
}
