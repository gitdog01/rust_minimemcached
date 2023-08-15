use std::collections::HashMap;
use std::io::{self, Write};
mod command;

struct Minimemcached {
    items: HashMap<String, i32>,
}

fn main() {
    let mut mm: Minimemcached = Minimemcached {
        items: HashMap::new(),
    };

    mm.items.insert("first_key".to_string(), 42);
    mm.items.insert("second_key".to_string(), 123);

    command_line(mm);

    println!("hello");
}

fn command_line(mm : Minimemcached) {
    loop {
        print!("> ");  // 프롬프트 출력
        io::stdout().flush().unwrap();  // 화면에 바로 표시되게 하기 위해 출력 버퍼를 flush합니다.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let buffer = input.trim();  // 입력의 앞뒤 공백 제거
                // get은 {커맨드} {키} {값} 형태로 입력받습니다.
                let mut iter = buffer.split_whitespace();
                let cmd = iter.next();
                match cmd {
                    Some("get") => {
                        let key = iter.next();
                        match key {
                            Some(key) => {
                                let value = command::getData(mm, key.to_string());
                                println!("key: {}, value: {}", key, value);
                                }
                            None => {
                                println!("key not found");
                            }
                        }
                    }
                    Some("set") => {
                        let key = iter.next();
                        let value = iter.next();
                        match (key, value) {
                            (Some(key), Some(value)) => {
                                println!("key: {}, value: {}", key, value);
                            }
                            _ => {
                                println!("key or value not found");
                            }
                        }
                    }
                    Some("quit") => {
                        println!("bye");
                        break;
                    }
                    _ => {
                        println!("unknown command");
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}