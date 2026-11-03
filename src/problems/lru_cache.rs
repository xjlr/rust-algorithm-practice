use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCache {
    capacity: usize,
    map: HashMap<i32, usize>,
    nodes: Vec<Node>,
    free_indices: Vec<usize>,
    head: Option<usize>, // Legfrissebb (Most Recently Used)
    tail: Option<usize>, // Legrégebbi (Least Recently Used)
}

impl LruCache {
    pub fn new(capacity: i32) -> Self {
        let cap = capacity.max(0) as usize;
        Self {
            capacity: cap,
            map: HashMap::with_capacity(cap),
            nodes: Vec::with_capacity(cap),
            free_indices: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.map.get(&key) {
            self.move_to_head(idx);
            self.nodes[idx].value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }

        if let Some(&idx) = self.map.get(&key) {
            self.nodes[idx].value = value;
            self.move_to_head(idx);
            return;
        }

        if self.map.len() >= self.capacity {
            self.evict_tail();
        }

        let new_idx = if let Some(idx) = self.free_indices.pop() {
            self.nodes[idx] = Node {
                key,
                value,
                prev: None,
                next: None,
            };
            idx
        } else {
            self.nodes.push(Node {
                key,
                value,
                prev: None,
                next: None,
            });
            self.nodes.len() - 1
        };

        self.push_front(new_idx);
        self.map.insert(key, new_idx);
    }

    fn detach(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        if let Some(p) = prev {
            self.nodes[p].next = next;
        } else {
            self.head = next;
        }

        if let Some(n) = next {
            self.nodes[n].prev = prev;
        } else {
            self.tail = prev;
        }

        self.nodes[idx].prev = None;
        self.nodes[idx].next = None;
    }

    fn push_front(&mut self, idx: usize) {
        self.nodes[idx].prev = None;
        self.nodes[idx].next = self.head;

        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(idx);
        } else {
            self.tail = Some(idx);
        }

        self.head = Some(idx);
    }

    fn move_to_head(&mut self, idx: usize) {
        if self.head == Some(idx) {
            return;
        }
        self.detach(idx);
        self.push_front(idx);
    }

    fn evict_tail(&mut self) {
        if let Some(old_tail) = self.tail {
            self.detach(old_tail);
            self.map.remove(&self.nodes[old_tail].key);
            self.free_indices.push(old_tail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LruCache;

    #[test]
    fn basic_eviction() {
        let mut cache = LruCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);

        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);

        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn updating_existing_key_refreshes_recency() {
        let mut cache = LruCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10);
        cache.put(3, 3);

        assert_eq!(cache.get(1), 10);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn get_refreshes_recency() {
        let mut cache = LruCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);

        cache.put(3, 3);

        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), -1);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LruCache::new(1);

        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);

        cache.put(2, 2);

        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn repeated_eviction_reuses_slots() {
        let mut cache = LruCache::new(2);

        cache.put(1, 10);
        cache.put(2, 20);
        cache.put(3, 30);
        cache.put(4, 40);
        cache.put(5, 50);

        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), -1);
        assert_eq!(cache.get(4), 40);
        assert_eq!(cache.get(5), 50);
    }

    #[test]
    fn update_does_not_increase_size_or_evict() {
        let mut cache = LruCache::new(2);

        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(2, 20);

        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(2), 20);
    }    
}
