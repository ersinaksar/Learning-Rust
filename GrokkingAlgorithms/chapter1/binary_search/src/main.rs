#![allow(dead_code)] //this line suppresses compiler warnings
fn main() {
    let my_list = [
        1, 2, 3, 4, 5, 6, 7, 8, 9,
        10, 20, 47, 59, 63, 75, 88,
        99, 107, 120, 133, 155, 162,
        176,188, 199, 200, 210, 222,
    ];
    let target: i32 = 120;
    if let Some(result) = binary_search(&my_list, my_list.len(), &target) {
        println!("{} found at index {}", target, result);
    } else {
        println!("{} not found.", target);
    }
}

fn binary_search(a: &[i32], len: usize, target_value: &i32) -> Option<usize> {
    let mut low: i8 = 0;
    let mut high: i8 = len as i8 - 1;

    while low <= high {
        let mid = ((high - low) / 2) + low;
        let mid_index = mid as usize;
        let val = &a[mid_index];

        if val == target_value {
            return Some(mid_index);
        }

        if val < target_value {
            low = mid + 1;
        }

        if val > target_value {
            high = mid - 1;
        }
    }
    None
}
