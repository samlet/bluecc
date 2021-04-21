use ink_env::Hash;

pub fn get_hash_id(input: &[u8]) -> Hash{
    let mut hash_output = [0x00_u8; 32];
    ink_env::hash_bytes::<ink_env::hash::Blake2x256>(input, &mut hash_output);
    let hash_id=Hash::from(hash_output);
    hash_id
}
