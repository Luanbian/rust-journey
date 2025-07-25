pub fn main() {
    bubble_sort();
    selection_sort();
}

fn bubble_sort() {
    let mut arr: [u32; 8] = [5, 3, 1, 4, 2, 14, 11, 9];
    loop {
        let mut swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    println!("{:?}", arr);
}

fn selection_sort() {
    let mut arr: [u32; 8] = [5, 3, 1, 4, 2, 14, 11, 9];
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
    println!("{:?}", arr);
}
