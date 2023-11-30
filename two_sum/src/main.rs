use std::collections::HashSet;
fn main() {
    let target = 6;
    let array = vec![4, 2, 5, 7];
    assert_eq!(two_sum(array, target), Some((4, 2)));
}

fn two_sum(list: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let mut map: HashSet<i32> = HashSet::new();

    for value in list {
        let compliment = target - value;

        if let Some(key) = map.get(&compliment) {
            return Some(((value, compliment)));
        } else {
            map.insert(value);
        }
    }
    None
}
