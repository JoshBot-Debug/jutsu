use super::Segment;

pub struct Find {
    value: String,
}

impl Segment for Find {
    fn data(&self) -> Vec<u8> {
        let length = match u8::try_from(self.value.len()) {
            Ok(v) => {
                if v > 254 {
                    error("Find has too many characters. Maximum is 254".to_string())
                }
                v
            }
            Err(_) => error("Find has too many characters. Maximum is 254".to_string()),
        };

        let mut data = Vec::with_capacity(length.into());

        let mut buf = self.value.clone().into_bytes();

        data.push(length);
        data.append(&mut buf);
        data
    }
}

impl Find {
    pub fn new(value: &str) -> Self {
        Self {value: value.to_string()}
    }
}

fn error(message: String) -> ! {
    eprintln!("{message}");
    std::process::exit(1)
}
