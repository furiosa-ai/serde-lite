use std::{
    collections::{BTreeMap, HashMap},
    hash::{Hash, Hasher},
};

use serde_lite::{intermediate, Deserialize};
use serde_lite_derive::Deserialize;

#[test]
fn test_deserialize_map_str_key() {
    let input = intermediate!({
        "foo": 3,
        "bar": 4,
    });

    let map: HashMap<String, usize> = HashMap::deserialize(&input).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map["foo"], 3);
    assert_eq!(map["bar"], 4);
}

#[test]
fn test_deserialize_map_int_key() {
    let input = intermediate!({
        "0": "foo",
        "4": "bar",
    });

    let map: HashMap<u32, String> = HashMap::deserialize(&input).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map[&0], "foo");
    assert_eq!(map[&4], "bar");
}

#[test]
fn test_deserialize_map_newtype_key() {
    let input = intermediate!({
        "42": 42,
        "-75": 75,
    });

    #[derive(Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    struct Idx(i64);

    let map: BTreeMap<Idx, i32> = BTreeMap::deserialize(&input).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map[&Idx(42)], 42);
    assert_eq!(map[&Idx(-75)], 75);
}

#[test]
fn test_deserialize_map_custom_float_key() {
    let input = intermediate!({
        "42": 42,
        "-75": 75,
    });

    #[derive(Deserialize)]
    struct Idx(f32);

    impl PartialEq for Idx {
        fn eq(&self, other: &Self) -> bool {
            self.0.to_ne_bytes() == other.0.to_ne_bytes()
        }
    }
    impl Eq for Idx {}
    impl Hash for Idx {
        fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
        {
            state.write(&self.0.to_ne_bytes());
        }
    }

    let map: HashMap<Idx, i32> = HashMap::deserialize(&input).unwrap();

    assert_eq!(map.len(), 2);
    assert_eq!(map[&Idx(42.)], 42);
    assert_eq!(map[&Idx(-75.)], 75);
}
