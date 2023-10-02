use std::io::SeekFrom;
use std::io::Seek;

use crate::character::Character;
use crate::constants::Mode;


#[derive(Debug)]
pub struct File{
    position: u64,
    file_length: u64,
    file: std::fs::File,
    current_character: [u8; 4],
    pub file_path: String,
    mode: Mode,
}

impl File{
    pub fn new(file_path: &String, mode: Mode) -> Self{
        use std::io::Read;

        let mut file = std::fs::File::open(file_path).expect(format!(
            "{mode}: File Error -> Can't open the file `{}`.",
            file_path).as_str());

        let file_length = file.metadata().expect(format!(
            "{mode}: File Error -> Failed to get file metadata `{}`.",
            file_path).as_str()).len();

        /* Read First Character */
        let mut first_char = [0; 1];
        file.read(&mut first_char).expect(format!(
            "{mode}: File Error -> Error in reading character `{}`.",
            file_path).as_str());

        let mut current_character = [0; 4];
        let mut position = 0;

        if first_char[0] < 127{
            position += 1;
            current_character[0] = first_char[0];
        }
        else if first_char[0] < 224{  // 2 Digits
            let mut second_char = [0; 1];
            file.read(&mut second_char).expect(format!(
                "{mode}: File Error -> Error in reading character `{}`.",
                file_path).as_str());

            position += 2;
            current_character = [first_char[0], second_char[0], 0, 0];
        }
        else if first_char[0] < 240{ // 3 Digits
            let mut char_arr = [0; 2];
            file.read(&mut char_arr).expect(format!(
                "{mode}: File Error -> Error in reading character `{}`.",
                file_path).as_str());

            position += 3;
            current_character = [first_char[0], char_arr[0], char_arr[1], 0];
        }
        else{ // 4 Digits
            let mut char_arr = [0; 3];
            file.read(&mut char_arr).expect(format!(
                "{mode}: File Error -> Error in reading character `{}`.",
                file_path).as_str());

            position += 4;
            current_character = [
                first_char[0], char_arr[0], char_arr[1], char_arr[2]];
        }

        return File{
            position,
            file_length,
            file,
            current_character,
            file_path: file_path.clone(),
            mode
        };
    }

    pub fn create_new(file_path: String, mode: Mode) -> Result<Self, std::io::Error>{
        let file = std::fs::File::create(&file_path)?;
        return Ok(File{
            position: 0,
            file_length: 0,
            file,
            current_character: [0; 4],
            file_path,
            mode
        })
    }
}

impl File{
    /* Implement The Peek Functionality With 0 and 1 Only */
    pub fn peek(&mut self, index: u64) -> Character{
        use std::io::Read;

        if index == 0{
            return Character::new(self.current_character, self.mode.clone());
        }

        let mut first_char = [0; 1];
        self.file.read(&mut first_char).expect(format!(
            "{}: File Error -> Error in reading character `{}`.",
            self.mode, self.file_path).as_str());

        if first_char[0] < 127{
            self.file.seek(SeekFrom::Start(self.position)).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_length).as_str());

            return Character::new([first_char[0], 0, 0, 0], self.mode.clone());
        }
        else if first_char[0] < 224{
            let mut second_char = [0; 1];
            self.file.read(&mut second_char).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.file.seek(SeekFrom::Start(self.position)).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_length).as_str());

            return Character::new([
                first_char[0], second_char[0], 0, 0], self.mode.clone());
        }
        else if first_char[0] < 240{
            let mut char_arr = [0; 2];
            self.file.read(&mut char_arr).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.file.seek(SeekFrom::Start(self.position)).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_length).as_str());

            return Character::new([
                first_char[0], char_arr[0], char_arr[1], 0], self.mode.clone());
        }
        else {
            let mut char_arr = [0; 3];
            self.file.read(&mut char_arr).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.file.seek(SeekFrom::Start(self.position)).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_length).as_str());

            return Character::new([
                first_char[0], char_arr[0], char_arr[1], char_arr[2]], self.mode.clone());
        }
    }

    /* Read UTF-8 Characters Which takes 4 bytes array */
    pub fn read(&mut self) -> Character{
        use std::io::Read;

        if self.position >= self.file_length{
            if self.current_character[0] != 0{
                let current = Character::new(
                    self.current_character, self.mode.clone());
                self.current_character = [0; 4];

                return current;
            }
            return Character::new([0, 0, 0, 0], self.mode.clone())
        }

        let mut first_char = [0; 1];
        self.file.read(&mut first_char).expect(format!(
            "{}: File Error -> Error in reading character `{}`.",
            self.mode, self.file_path).as_str());

        if first_char[0] < 127{
            self.position += 1;
            let current = Character::new(
                self.current_character, self.mode.clone());
            self.current_character = [first_char[0], 0, 0, 0];

            return current;
        }
        else if first_char[0] < 224{  // 2 Digits
            let mut second_char = [0; 1];
            self.file.read(&mut second_char).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.position += 2;
            let current = Character::new(
                self.current_character, self.mode.clone());
            self.current_character = [first_char[0], second_char[0], 0, 0];

            return current;
        }
        else if first_char[0] < 240{ // 3 Digits
            let mut char_arr = [0; 2];
            self.file.read(&mut char_arr).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.position += 3;
            let current = Character::new(
                self.current_character, self.mode.clone());
            self.current_character = [
                first_char[0], char_arr[0], char_arr[1], 0];

            return current;
        }
        else{ // 4 Digits
            let mut char_arr = [0; 3];
            self.file.read(&mut char_arr).expect(format!(
                "{}: File Error -> Error in reading character `{}`.",
                self.mode, self.file_path).as_str());

            self.position += 4;
            let current = Character::new(
                self.current_character, self.mode.clone());
            self.current_character = [
                first_char[0], char_arr[0], char_arr[1], char_arr[2]];

            return current;
        }
    }

    pub fn write(&mut self, data: String){
        use std::io::Write;

        self.file.write_all(data.as_bytes()).expect(format!(
            "{}: File Error -> Error in Writing Data Into File.", self.mode).as_str());
    }

    pub fn writeln(&mut self, data: String){
        use std::io::Write;

        let mut data = data;
        data.push_str("\n");

        self.file.write_all(data.as_bytes()).expect(format!(
            "{}: File Error -> Error in Writing Data Into File.",
            self.mode).as_str());
    }

    pub fn delete_file(file_path: String, mode: Mode){
        std::fs::remove_file(file_path).expect(format!(
            "{}: File Error -> Error in deleting file.", mode).as_str());
    }

    pub fn get_stream_position(&mut self) -> u64{
        return self.file.stream_position().expect(format!(
            "{}: File Error -> Failed to get stream position.",
            self.mode).as_str());
    }

    pub fn rewrite_line(&mut self, stream_position: u64, data: String){
        self.file.seek(SeekFrom::Start(stream_position)).expect(format!(
            "{}: File Error -> Failed to set stream position.",
            self.mode).as_str());

        self.writeln(data);

        self.file.seek(SeekFrom::End(0)).expect(format!(
            "{}: File Error -> Failed to set stream position.",
            self.mode).as_str());
    }
}
