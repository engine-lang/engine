use crate::constants::Mode;


#[derive(Debug, Clone, PartialEq)]
pub struct Character{
    code: [u8; 4],
    string: String,
}

impl Character{
    pub fn new(code: [u8; 4], mode: Mode) -> Self{
        use std::str::from_utf8;

        if code[0] < 128{
            return Character{
                string: String::from(
                    from_utf8(&code[0..1])
                    .expect(format!(
                        "{mode}: Token Error -> Not valid utf-8 character.").as_str())),
                code,
            }
        }
        else if code[0] < 224{
            return Character{
                string: String::from(
                    from_utf8(&code[0..2])
                    .expect(format!(
                        "{mode}: Token Error -> Not valid utf-8 character.").as_str())),
                code,
            }
        }
        else if code[0] < 240{
            return Character{
                string: String::from(
                    from_utf8(&code[0..3])
                    .expect(format!(
                        "{mode}: Token Error -> Not valid utf-8 character.").as_str())),
                code,
            }
        }
        else{
            return Character{
                string: String::from(
                    from_utf8(&code[0..4])
                    .expect(format!(
                        "{mode}: Token Error -> Not valid utf-8 character.").as_str())),
                code,
            }
        }
    }
}

impl Character{
    pub fn is_digit(&self) -> bool{
        return self.code[0] > 47 && self.code[0] < 58
    }

    pub fn is_alpha(&self) -> bool{
        return self.code[0] > 64 && self.code[0] < 91 || self.code[0] > 96 && self.code[0] < 123
    }

    pub fn is_space(&self) -> bool{
        return self.code[0] == 32 || self.code[0] == 9 || self.code[0] == 13
    }

    pub fn is_newline(&self) -> bool{
        return self.code[0] == 10
    }

    pub fn is_eof(&self) -> bool{
        return self.code[0] == 0
    }

    pub fn is_ascii(&self) -> bool{
        return self.code[0] < 128;
    }

    pub fn to_string(&self) -> &String{
        return &self.string;
    }
}
