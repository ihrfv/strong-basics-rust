use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

// O(m * log(n))
// binary heap pop and push are O(log(n))
// and it's don m time where m = customers.len()
fn queue_time_via_min_heap(customers: &[u32], n: u32) -> u32 {
    // A Min-Heap to track the time each till becomes free
    let mut tills = BinaryHeap::from(vec![Reverse(0); n as usize]);

    for &customer in customers {
        // Pop the till that finishes soonest
        if let Some(Reverse(earliest_finish)) = tills.pop() {
            // Add the customer and push it back
            tills.push(Reverse(earliest_finish + customer));
        }
    }

    // The total time is the maximum value remaining in the heap
    tills.into_iter().map(|Reverse(t)| t).max().unwrap_or(0)
}

// 0(m * n)
// because pop_front is O(1) and insert is O(min(i, n - i)) which is O(n)
// note binary search is O(log n) but the biggest impact on
// the complexity is data move due to insert
// and it's done for all customers
fn queue_time(customers: &[u32], n: u32) -> u32 {
    if customers.is_empty() {
        return 0;
    }

    if n == 1 {
        return customers.iter().sum();
    }

    if n as usize > customers.len() {
        return *customers
            .iter()
            .max()
            .expect("Array shouldn't be empty at this point");
    }

    let mut queue_time = 0;
    let mut tills: VecDeque<u32> = vec![0; n as usize].into();
    for customer in customers {
        let updated_val = tills.pop_front().unwrap_or(0) + customer;
        let indx = tills.partition_point(|&x| x < updated_val);
        tills.insert(indx, updated_val);
    }
    queue_time += tills.pop_back().unwrap_or(0);
    queue_time
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(c: &[u32], n: u32, expected: u32) {
        let actual = queue_time_via_min_heap(c, n);
        assert!(
            actual == expected,
            "With customers = {c:?}, n = {n}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&[], 1, 0);
        dotest(&[5], 1, 5);
        dotest(&[2], 5, 2);
        dotest(&[1, 2, 3, 4, 5], 1, 15);
        dotest(&[1, 2, 3, 4, 5], 100, 5);
        dotest(&[2, 2, 3, 3, 4, 4], 2, 9);
    }
}
