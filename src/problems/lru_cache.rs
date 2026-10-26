pub struct LruCache {
    _todo: (),
}

impl LruCache {
    pub fn new(capacity: i32) -> Self {
        let _ = capacity;
        todo!("Implement LruCache::new")
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let _ = key;
        todo!("Implement LruCache::get")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let _ = (key, value);
        todo!("Implement LruCache::put")
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
}
