/// A comparision sort
pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let n: usize = arr.len();

    for i in 0..n - 1 {
        let mut min_idx: usize = i;

        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            arr.swap(min_idx, i)
        }
    }
}

/// A comparision sort
pub fn bubble_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let n: usize = arr.len();
    let mut swapped: bool = false;

    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);

                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

/// A comparision sort
pub fn insertion_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let n: usize = arr.len();

    for i in 1..n {
        let key: T = arr[i];
        let mut j: usize = i - 1;

        while arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }
        
        arr[j + 1] = key;
    }
}

/// A comparision sort
pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    fn merge_sort_internal<T: PartialOrd + Copy>(arr: &mut [T], begin: usize, end: usize) {
        fn merge<T: PartialOrd + Copy>(arr: &mut [T], left: usize, mid: usize, right: usize) {
            let left_arr_len: usize = mid - left + 1;
            let right_arr_len: usize = right - mid;
        
            let mut left_arr: Vec<T> = Vec::with_capacity(left_arr_len);
            let mut right_arr: Vec<T> = Vec::with_capacity(right_arr_len);
        
            for i in 0..left_arr_len {
                left_arr.push(arr[left + i])
            }
        
            for i in 0..right_arr_len {
                right_arr.push(arr[mid + i + 1])
            }
        
            let mut left_index: usize = 0;
            let mut right_index: usize = 0;
        
            let mut merged_index = left;
        
            while left_index < left_arr_len && right_index < right_arr_len {
                if left_arr[left_index] <= right_arr[right_index] {
                    arr[merged_index] = left_arr[left_index];
        
                    left_index += 1;
                } else {
                    arr[merged_index] = right_arr[right_index];
        
                    right_index += 1;
                }
        
                merged_index += 1;
            }
        
            while left_index < left_arr_len {
                arr[merged_index] = left_arr[left_index];
        
                left_index += 1;
                merged_index += 1;
            }
        
            while right_index < right_arr_len {
                arr[merged_index] = right_arr[right_index];
        
                right_index += 1;
                merged_index += 1;
            }
        }

        if begin >= end {
            return;
        }
    
        let mid = begin + (end - begin) / 2;
        merge_sort_internal(arr, begin, mid);
        merge_sort_internal(arr, mid + 1, end);
    
        merge(arr, begin, mid, end);
    }

    merge_sort_internal(arr, 0, arr.len() - 1)
}

/// A comparision sort
pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    fn quick_sort_internal<T: PartialOrd + Copy>(arr: &mut [T], low: usize, high: usize) {
        fn partition<T: PartialOrd + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
            let pivot: T = arr[high];

            let mut i: i64 = low as i64 - 1;

            for j in low..=high - 1 {
                if arr[j] < pivot {
                    i += 1;

                    arr.swap(i as usize, j);
                }
            }

            arr.swap((i + 1) as usize, high);

            (i + 1) as usize
        }

        if low < high {
            let pi: usize = partition(arr, low, high);

            if pi > 0 {
                quick_sort_internal(arr, low, pi - 1);
            }
            quick_sort_internal(arr, pi + 1, high);
        }
    }

    quick_sort_internal(arr, 0, arr.len() - 1);
}


/// A comparision sort
pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    fn heapify<T: PartialOrd>(arr: &mut [T], n: usize, i: usize) {
        let mut largest: usize = i;
    
        let left: usize = 2 * i + 1;
        let right: usize = 2 * i + 2;
    
        if left < n && arr[left] > arr[largest] {
            largest = left;
        }
    
        if right < n && arr[right] > arr[largest] {
            largest = right;
        }
    
        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    let n: usize = arr.len();

    for i in (0..=n / 2 - 1).rev() {
        heapify(arr, n, i);
    }

    for i in (0..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

/// A non-comparision sort
pub fn counting_sort<T: PartialOrd + Copy, F: Fn(&T) -> usize>(array: &mut [T], key_extractor: F) {
    let max = array
        .iter()
        .max_by_key(|x| key_extractor(x))
        .map(&key_extractor)
        .unwrap_or(0);

    let mut count = vec![0; max + 1];

    for item in array.iter() {
        let key = key_extractor(item);
        count[key] += 1;
    }

    for i in 1..count.len() {
        count[i] += count[i - 1];
    }

    let mut result = vec![array[0]; array.len()];

    for i in (0..array.len()).rev() {
        let item = array[i];
        let key = key_extractor(&item);
        let position = count[key] - 1;
        result[position] = item;
        count[key] -= 1;
    }

    array.copy_from_slice(&result);
}

/// A non-comparison sort
pub fn radix_sort<T: PartialOrd + Copy + From<u8> + From<usize> + Into<usize>, F: Fn(&T) -> usize>(arr: &mut [T], radix_fn: F) {
    fn counting_sort_digit<T: PartialOrd + Copy + From<usize>, F: Fn(&T) -> usize>(arr: &mut [T], radix_fn: F, exp: usize) {
        let mut count: Vec<usize> = vec![0; 10];
        let mut output: Vec<T> = vec![arr[0]; arr.len()];
    
        for num in arr.iter() {
            let index: usize = radix_fn(num) / exp % 10;
            count[index] += 1;
        }
    
        for i in 1..10 {
            count[i] += count[i - 1];
        }
    
        for i in (0..arr.len()).rev() {
            let num: T = arr[i];
            let index: usize = radix_fn(&num) / exp % 10;
            let position: usize = count[index] - 1;
            output[position] = num;
            count[index] -= 1;
        }
    
        arr.clone_from_slice(&output);
    }

    fn get_max<T: PartialOrd + Copy>(arr: &[T]) -> T {
        let mut max: T = arr[0];
        for &num in arr.iter() {
            if num > max {
                max = num;
            }
        }
        max
    }

    let max: T = get_max(arr);
    let max_value: usize = max.into();

    let mut exp: usize = 1;
    while max_value / exp > 0 {
        counting_sort_digit(arr, &radix_fn, exp);
        exp *= 10;
    }
}

/// f64 only
pub fn bucket_sort(arr: &mut [f64]) {
    let n: usize = arr.len();
    
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); n];
  
    for &num in arr.iter() {
        let bi: usize = (n as f64 * num) as usize; // Index in bucket
        buckets[bi].push(num);
    }

    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
    }
  
    let mut index: usize = 0;
    for bucket in buckets.iter() {
        for &num in bucket {
            arr[index] = num;
            index += 1;
        }
    }
}