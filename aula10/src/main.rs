fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp = Vec::with_capacity(len);
    {
        let(left, right) = arr.split_at(mid);
        let mut i = 0;
        let mut j = 0;
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                temp.push(left[i]);
                i += 1;
            } else {
                temp.push(right[j]);
                j +=1;
            }
        }
        temp.extend_from_slice(&left[i..]);
        temp.extend_from_slice(&right[j..]);
    }
    arr.copy_from_slice(&temp);
}

fn main() {
    let mut array = [12, 11, 13, 5, 6, 7];
    println!("Array original: {:?}", array);
    merge_sort(&mut array);
    println!("Merge Sort: {:?}", array);
}
