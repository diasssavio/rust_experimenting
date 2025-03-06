use crate::extras::Order;

mod extras;

fn main() {
    println!("Hello, world!");

    let mut numbers = vec![5, 2, 9, 1, 5, 6];

    println!("Before sorting: {:?}", numbers);
    extras::sort(& mut numbers, Order::Ascending);
    println!("After sorting (ascending): {:?}", numbers);
    extras::sort(& mut numbers, Order::Descending);
    println!("After sorting (descending): {:?}", numbers);
}
