trait DBOps<K, V> {
    fn get(k: K) -> V;
}