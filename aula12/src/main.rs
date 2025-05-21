fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        if arr[high] == arr[low] {
            return if arr[low] == target { Some(low) } else { None };
        }
        
        let pos = low + (((target - arr[low]) as usize * (high - low))
                        / ((arr[high] - arr[low]) as usize));
        if pos >= arr.len() {
            break;
        }
        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            low = pos + 1;
        } else {
            if pos == 0 { break }
            high = pos - 1;
        }
        }
        None
}

fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let target = 70;
    match interpolation_search(&arr, target) {
        Some(index) => println!("Elemento {} encontrado no indice {}.", target, index),
        None => println!("Elemento {} nao encontrado.", target)
    }

}
