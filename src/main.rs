use rand::Rng;

trait SortTraits: Clone + std::fmt::Debug + std::fmt::Display + std::cmp::PartialOrd {}
impl<T: Clone + std::fmt::Debug + std::fmt::Display + std::cmp::PartialOrd> SortTraits for T {}

#[allow(dead_code)]
fn bubble_sort<T: SortTraits>(x: &Vec<T>) -> Vec<T> {
    let mut y: Vec<T> = Vec::new();
    for i in 0..x.len() {
        y.push(x[i].clone());
    }
    for i in 0..y.len() {
        for j in i..y.len() {
            if y[i] > y[j] {
                let t = y[i].clone();
                y[i] = y[j].clone();
                y[j] = t;
            }
        }
    }
    y
}
#[allow(dead_code)]
fn bubble_sort_slice<T: SortTraits>(y: &mut [T]) -> &[T] {
    for i in 0..y.len() {
        for j in i..y.len() {
            if y[i] > y[j] {
                let t = y[i].clone();
                y[i] = y[j].clone();
                y[j] = t;
            }
        }
    }
    y
}
#[allow(dead_code)]
fn quick_sort<T: SortTraits>(x: &Vec<T>) -> Vec<T> {
    // Sorts a (portion of an) array, divides it into partitions, then sorts those
    fn quicksort<T: SortTraits>(a: &mut Vec<T>, lo: i32, hi: i32) {
        // Ensure indices are in correct order
        if lo >= hi || lo < 0 {
            return;
        }

        // Partition array and get the pivot index
        let l = usize::try_from(lo).unwrap();
        let h = usize::try_from(hi).unwrap();
        let p = partition(a, l, h);

        // Sort the two partitions
        quicksort(a, lo, p - 1); // Left side of pivot
        quicksort(a, p + 1, hi); // Right side of pivot
    }
    // Divides array into two partitions
    fn partition<T: SortTraits>(y: &mut Vec<T>, lo: usize, hi: usize) -> i32 {
        let pivot = y[hi].clone(); // Choose the last element as the pivot
                                   // Temporary pivot index
        let mut i = lo;

        for j in lo..hi {
            // If the current element is less than or equal to the pivot
            if y[j] <= pivot {
                // Move the temporary pivot index forward
                // Swap the current element with the element at the temporary pivot index
                let t = y[i].clone();
                y[i] = y[j].clone();
                y[j] = t;
                i = i + 1;
            }
        }
        // Move the pivot element to the correct pivot position (between the smaller and larger elements)
        let t = y[i].clone();
        y[i] = y[hi].clone();
        y[hi] = t;
        i = i + 1;
        i32::try_from(i).unwrap() - 1 // the pivot index
    }

    let mut y: Vec<T> = Vec::new();
    for i in 0..x.len() {
        y.push(x[i].clone());
    }
    quicksort(&mut y, 0, i32::try_from(x.len()).unwrap() - 1);
    y
}

#[allow(dead_code)]
fn quicksort<T: SortTraits>(a: &mut [T]) -> &[T] {
    // Divides array into two partitions
    fn partition<T: SortTraits>(y: &mut [T]) -> usize {
        let hi = y.len() - 1;
        let pivot = y[hi].clone(); // Choose the last element as the pivot
                                   // Temporary pivot index
        let mut i = 0;
        for j in 0..hi {
            // If the current element is less than or equal to the pivot
            if y[j] <= pivot {
                y.swap(i, j);
                i = i + 1;
            }
        }
        // Move the pivot element to the correct pivot position (between the smaller and larger elements)
        y.swap(i, hi);
        i + 1
    }
    if a.len() <= 1 {
        return a;
    }
    // Partition array and get the pivot index
    let p = partition(a);
    let hi = a.len();
    // Sort the two partitions
    quicksort(&mut a[0..p - 1]); // Left side of pivot
    quicksort(&mut a[p..hi]); // Right side of pivot
    a
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    let i = (0..20).map(|_| rng.gen_range(0..100)).collect::<Vec<i32>>();
    println!("S={:?}", &i);
    let mut x = i.clone();
    println!("R={:?}", bubble_sort_slice(&mut x[0..20]));
    println!("S={:?}", quick_sort(&i));
    println!("Q={:?}", quicksort(&mut i.clone()));
}
