use std::io::{
    BufRead,
    BufReader
};

use crate::constants::Mode;


#[derive(Debug)]
pub struct File{
    reader: Option<BufReader<std::fs::File>>,
}

impl File{
    pub fn open(file_path: &String) -> Self{
        let file = std::fs::File::open(file_path).expect(format!(
            "{}: File Error -> Can't open the file `{}`.",
            Mode::VirtualMachine, file_path).as_str());

        return File{
            reader: Some(BufReader::new(file.try_clone().expect(format!(
                "{}: File Error -> Failed Construct file reader `{}`.",
                Mode::VirtualMachine, file_path).as_str()))),
        };
    }
}

impl File{
    pub fn read_line(&mut self) -> Result<(String, usize), std::io::Error>{
        let mut line = String::new();

        let len = self.reader.as_mut().unwrap().read_line(&mut line)?;

        if line.chars().last() == Some('\n'){
            return Ok((String::from(&line[0..line.len()-1]), len));
        }

        return Ok((line, len));
    }
}
