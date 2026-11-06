use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest {
    k: i32,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest_element_in_a_stream = KthLargest {
            k,
            heap: BinaryHeap::new(),
        };
        for &num in &nums {
            kth_largest_element_in_a_stream.add(num);
        }
        kth_largest_element_in_a_stream
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k as usize {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn basic_case() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);

        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }

    #[test]
    fn starts_with_empty_stream() {
        let mut kth = KthLargest::new(1, vec![]);

        assert_eq!(kth.add(-3), -3);
        assert_eq!(kth.add(-2), -2);
        assert_eq!(kth.add(-4), -2);
        assert_eq!(kth.add(0), 0);
        assert_eq!(kth.add(4), 4);
    }

    #[test]
    fn duplicate_values() {
        let mut kth = KthLargest::new(2, vec![5, 5, 5]);

        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(6), 5);
        assert_eq!(kth.add(7), 6);
    }

    #[test]
    fn negative_values() {
        let mut kth = KthLargest::new(2, vec![-10, -7, -3]);

        assert_eq!(kth.add(-8), -7);
        assert_eq!(kth.add(-2), -3);
        assert_eq!(kth.add(-1), -2);
    }
}
