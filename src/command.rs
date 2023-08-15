use crate::Minimemcached;

enum command {
    Get(String),
    Set(String),
}

pub(crate) fn getData(mm: Minimemcached,key: String ) {
    mm.items.get(&key);
}

pub(crate) fn setData(mut mm: Minimemcached,key: String, value: i32) {
    mm.items.insert(key, value);
}