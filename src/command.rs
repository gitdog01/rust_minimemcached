use crate::{Minimemcached, action};

pub(crate) fn get(mm: &Minimemcached,key: Option<&str>) {
    match key {
        Some(key) => {
            let value: Option<String> = action::get_data(&mm, key.to_string());
            // 키로 값 가져오기
            println!("{:?}",value)
            // print 할때는 {:?} 를 사용하세요. 
            }
        None => {
            println!("key not found");
        }
    }
}

pub(crate) fn set(mm:&mut  Minimemcached,key:Option<&str> , value: Option<&str>){
    match (key, value) {
        (Some(key), Some(value)) => {
            // 임시값 생성
            action::set_data(mm, key.to_owned(), value.to_string())
        }
        _ => {
            println!("key or value not found");
        }
    }
}

pub(crate) fn flush(mm:&mut Minimemcached){
    println!("flush!");
    action::flush_data(mm);
}

pub(crate) fn quit(){
    println!("bye");
    std::process::exit(0)
}

