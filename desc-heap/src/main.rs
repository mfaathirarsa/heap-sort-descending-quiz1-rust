fn min_heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut terkecil = i;
    let kiri = 2 * i + 1;
    let kanan = 2 * i + 2;

    if kiri < n && arr[kiri] < arr[terkecil] {
        terkecil = kiri;
    }
    if kanan < n && arr[kanan] < arr[terkecil] {
        terkecil = kanan;
    }

    if terkecil != i {
        arr.swap(i, terkecil);
        min_heapify(arr, n, terkecil);
    }
}

fn build_min_heap(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        min_heapify(arr, n, i);
    }
}

fn heap_sort_desc(arr: &mut Vec<i32>) {
    let n = arr.len();
    build_min_heap(arr);

    for i in (1..n).rev() {
        arr.swap(0, i);
        min_heapify(arr, i, 0);
    }
}

fn main() {
    let mut arr = vec![4, 10, 3, 5, 1];
    heap_sort_desc(&mut arr);
    println!("Array setelah diurutkan (descending): {:?}", arr);
}