fn main() {
	let mut arr = [ 27, 37, 28, 02, 74, 83, 92];
	let mut arr_len = arr.len();
	let mut sorted = false;
	println!("Messy Array: {:?}", arr);
	while !sorted {
		sorted = true;
		for i in 0..arr_len - 1 {
			if arr[i] > arr[i + 1] {
				arr.swap(i, i + 1);
				sorted = false;
			}
		}
		arr_len -= 1;
	}
	println!("Sorted Array: {:?}", arr);
}