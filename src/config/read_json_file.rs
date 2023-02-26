#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    name: String,
    data:Vec<u8>,
    state:FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name:String::from(name),
            data:Vec::new(),
            state:FileState::Closed
        }
    }
    fn new_with_data(name:&str,data:Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    fn read(self: &File,save_to:&mut Vec<u8>) -> Result<usize,String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

fn open(mut f:File) -> Result<File,String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f:File) -> Result<File,String> {
    f.state = FileState::Closed;
    Ok(f)
}
