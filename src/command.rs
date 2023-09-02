use crate::Minimemcached;

fn get(mm: &Minimemcached,key: String);

fn set(mm:&mut  Minimemcached,key: String, value: String);

fn flush(mm:&mut Minimemcached);

fn quit();

