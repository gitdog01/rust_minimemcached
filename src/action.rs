use std::collections::HashMap;

use crate::Minimemcached;

pub(crate) fn get_data(mm: &Minimemcached,key: String) -> Option<String> {
    mm.items.get(&key).cloned()
}

pub(crate) fn set_data(mm:&mut  Minimemcached,key: String, value: String) {
    mm.items.insert(key, value);
}

pub(crate) fn flush_data(mm:&mut Minimemcached) {
    mm.items = HashMap::new();
}