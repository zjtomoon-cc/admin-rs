#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data:Vec<u8>,
    state:FileState,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name:String::from(name),
            data:Vec::new(),
            state:FileState::Closed
        }
    }
    pub fn new_with_data(name:&str,data:Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }
    pub fn read(self: &File,save_to:&mut Vec<u8>) -> Result<usize,String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

pub fn open(mut f:File) -> Result<File,String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f:File) -> Result<File,String> {
    f.state = FileState::Closed;
    Ok(f)
}

/*
关于公共函数和公共类，初始阶段先对每个函数公开，方便进行单元测试，后期使用统一的公共函数开放调用。
 */
