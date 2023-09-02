use std::collections::HashMap;
use std::io::{self, Write};
mod action;

struct Minimemcached {
    items: HashMap<String, String>,
}

fn main() {
    let mut mm: Minimemcached = Minimemcached {
        items: HashMap::new(),
    };

    println!("hello");

    command_line(&mut mm);

}

fn command_line(mm :&mut Minimemcached) {
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
                        // 커맨드가 get 일때
                        let key = iter.next();
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
                    Some("set") => {
                        let key = iter.next();
                        let value = iter.next();
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
                    Some("flush") => {
                        action::flush_data(mm);
                        println!("flush!")
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