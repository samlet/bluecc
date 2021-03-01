use scale::{Compact, Decode, Encode, HasCompact};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Encode, Decode)]
struct S {
    x: u32,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Encode, Decode)]
struct SSkip {
    #[codec(skip)]
    s1: u32,
    x: u32,
    #[codec(skip)]
    s2: u32,
}

#[test]
fn codec_vec_u8() {
    for v in [
        vec![0u8; 0],
        vec![0u8; 10],
        vec![0u8; 100],
        vec![0u8; 1000],
    ].iter() {
        let e = v.encode();
        assert_eq!(v, &Vec::<u8>::decode(&mut &e[..]).unwrap());
    }
}


#[test]
fn associated_type_bounds() {
    trait Trait {
        type EncodableType;
        type NonEncodableType;
    }

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Struct<T: Trait, Type> {
        field: (Vec<T::EncodableType>, Type),
    }

    #[derive(Debug, PartialEq)]
    struct TraitImplementor;

    struct NonEncodableType;

    impl Trait for TraitImplementor {
        type EncodableType = u32;
        type NonEncodableType = NonEncodableType;
    }

    let value: Struct<TraitImplementor, u64> = Struct { field: (vec![1, 2, 3], 42) };
    let encoded = value.encode();
    let decoded: Struct<TraitImplementor, u64> = Struct::decode(&mut &encoded[..]).unwrap();
    assert_eq!(value, decoded);
}

