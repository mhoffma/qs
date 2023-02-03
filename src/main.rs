use rand::Rng;

#[allow(dead_code)]
fn sort<T: Clone>(x: &Vec<T>) -> Vec<T> {
    let mut y: Vec<T> = Vec::new();
    for i in 0..x.len() {
        y.push(x[i].clone());
    }
    y
}

#[allow(dead_code)]
fn bubble_sort<T: Clone + std::cmp::PartialOrd>(x: &Vec<T>) -> Vec<T> {
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
fn quick_sort<T: Clone + std::fmt::Debug + std::cmp::PartialOrd>(x: &Vec<T>) -> Vec<T> {
    // Sorts a (portion of an) array, divides it into partitions, then sorts those
    fn quicksort<T: Clone + std::fmt::Debug + std::cmp::PartialOrd>(
        a: &mut Vec<T>,
        lo: i32,
        hi: i32,
    ) {
        // Ensure indices are in correct order
        if lo >= hi || lo < 0 {
            return;
        }

        // Partition array and get the pivot index
        //
        let l = usize::try_from(lo).unwrap();
        let h = usize::try_from(hi).unwrap();
        let p = partition(a, l, h);

        // Sort the two partitions
        quicksort(a, lo, p - 1); // Left side of pivot
        quicksort(a, p + 1, hi); // Right side of pivot
    }
    // Divides array into two partitions
    fn partition<T: Clone + std::fmt::Debug + std::cmp::PartialOrd>(
        y: &mut Vec<T>,
        lo: usize,
        hi: usize,
    ) -> i32 {
        let pivot = y[hi].clone(); // Choose the last element as the pivot
        println!("partition({:?} ({lo}..{hi}))", y);
        // Temporary pivot index
        let mut i = lo;

        for j in lo..hi - 1 {
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
        i32::try_from(i).unwrap() // the pivot index
    }

    let mut y: Vec<T> = Vec::new();
    for i in 0..x.len() {
        y.push(x[i].clone());
    }
    quicksort(&mut y, 0, i32::try_from(x.len()).unwrap() - 1);
    y
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    let i = (0..10).map(|_| rng.gen_range(0..20)).collect::<Vec<i32>>();
    println!("S={:?}", &i);
    println!("S={:?}", quick_sort(&i));
}
