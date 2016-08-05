#[cfg(test)]
#[macro_use]
extern crate meta;

/// This is ported from the Dart heap sort implementation
fn heap_sort<T: Ord>(a: &mut [T]) {
    let count = a.len();

    if count == 0 {
        return;
    }

    // first place 'a' in max-heap order
    heapify(a, count);

    let mut end = count - 1;
    while end > 0 {
        // swap the root (maximum value) of the heap with the
        // last element of the heap
        a.swap(0, end);

        // put the heap back in max-heap order
        sift_down(a, 0, end - 1);

        // decrement the size of the heap so that the previous
        // max value will stay in its proper place
        end -= 1;
    }
}

fn heapify<T: Ord>(a: &mut [T], count: usize) {
    if count < 2 {
        return;
    }

    // start is assigned the index in 'a' of the last parent node
    let mut start: i32 = (count as i32 - 2) / 2; // binary heap

    while start >= 0 {
        // sift down the node at index 'start' to the proper place
        // such that all nodes below the 'start' index are in heap order
        sift_down(a, start as usize, count - 1);
        start -= 1;
    }
}


fn sift_down<T: Ord>(a: &mut [T], start: usize, end: usize) {
    // end represents the limit of how far down the heap to shift
    let mut root = start;

    // while the root has at least one child
    while (root * 2 + 1) <= end {
        // root*2+1 points to the left child
        let mut child: usize = root * 2 + 1 as usize;

        // if the chile has a sibling and the child's value is less that its sibling's...
        if child + 1 <= end && a[child] < a[child + 1] {
            // .. then point to the right child instead
            child = child + 1;
        }

        // out of max-heap order
        if a[root] < a[child] {
            a.swap(root, child);
            // repeat to continue shifting down the child now
            root = child;
        } else {
            return;
        }
    }
}

pub fn main() {
    let mut arr = [1i32, 5, 2, 7, 3, 9, 4, 6, 8];
    heap_sort(&mut arr);
    println!("After sort: {:?}", arr);

    let mut arr = [1i32, 2, 3, 4, 5, 6, 7, 8, 9];
    heap_sort(&mut arr);
    println!("After sort: {:?}", arr);

    let mut arr = [9i32, 8, 7, 6, 5, 4, 3, 2, 1];
    heap_sort(&mut arr);
    println!("After sort: {:?}", arr);
}

#[cfg(test)]
mod tests {
    test_sort!(super::heap_sort);
}
