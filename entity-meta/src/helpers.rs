use ink_storage::collections::Vec as StorageVec;

/// Creates a storage vector from the given slice.
pub fn vec_from_slice(slice: &[u8]) -> StorageVec<u8> {
    slice.iter().copied().collect::<StorageVec<u8>>()
}

