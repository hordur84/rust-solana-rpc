use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

/// Data structure used to keep track of isolated SOL transfer instructions.
/// Contains `source`, `dest` and `signature` fields.
pub struct SData {
    pub source: String,
    pub dest: String,
    pub signature: String
}

impl Hash for SData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source.hash(state);
        self.dest.hash(state);
        self.signature.hash(state);
    }
}
/// Calculate hash. 
/// 
///  # Arguments
/// 
/// * `t` - Object to calculate hash for, must implement `Hash`.
/// ```
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}