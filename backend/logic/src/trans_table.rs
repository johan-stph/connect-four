
const SIZE: usize = (1 << 23) + 9;
pub struct TranspositionTable {
    keys: Vec<u64>,
    values: Vec<u8>,
}
impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            keys: vec![0; SIZE],
            values: vec![0; SIZE],
        }
    }

    fn index(&self, key: u64) -> usize {
        (key % (SIZE as u64)) as usize
    }

    /**
     * Store a value for a given key
     * @param key: must be less than key_size bits.
     * @param value: must be less than value_size bits. null (0) value is used to encode missing data
     */
    pub fn put(&mut self, key: u64, value: u8) {
        let pos = self.index(key);
        self.keys[pos] = key;
        self.values[pos] = value;
    }

    /**
     * Get the value of a key
     * @param key: must be less than key_size bits.
     * @return value_size bits value associated with the key if present, 0 otherwise.
     */
    pub fn get(&self, key: u64) -> Option<u8> {
        let pos = self.index(key);
        if self.keys[pos] == key {
            Some(self.values[pos])
        } else {
            None
        }
    }
}