pub use intercom::*;
use intercom::raw::HRESULT;
use libc::*;
use std::fs::File;
use std::io::{Write, Read};

com_library! {
    class Unzip
}

#[com_class(Unzip)]
struct Unzip {
}

impl Default for Unzip {
    fn default() -> Self {
        Unzip {  }
    }
}

#[com_interface]
impl Unzip {
    #[allow(non_snake_case)]
    pub fn Unzip(&mut self, src_path: *const c_char, dest_path: *const c_char) -> ComResult<i32> {
        let src_file_path = unsafe { CStr::from_ptr(src_path).to_string_lossy().into_owned() };
            let dest_file_path = unsafe { CStr::from_ptr(dest_path).to_string_lossy().into_owned() };

            // 打开源文件
            let mut src_file = match File::open(&src_file_path) {
                Ok(file) => file,
                Err(_) => return Err(ComError::new_hr(HRESULT::new(-1))), // 文件打开失败
            };

            // 创建目标文件
            let mut dest_file = match File::create(&dest_file_path) {
                Ok(file) => file,
                Err(_) => return Err(ComError::new_hr(HRESULT::new(-3))), // 目标文件创建失败
            };

            // 将解压后的文件内容写入目标文件
            let mut in_buffer = Vec::new();
            if src_file.read_to_end(&mut in_buffer).is_err() {
                return Err(ComError::new_hr(HRESULT::new(-4))); // 文件读取失败
            }

            let mut out_buffer = Vec::new();

            decode(&in_buffer, &mut out_buffer);

            if dest_file.write_all(&out_buffer).is_err() {
                return Err(ComError::new_hr(HRESULT::new(-5))); // 文件写入失败
            }
        Ok(0)
    }
}

fn decode(data: &[u8], buf: &mut Vec<u8>) -> usize {
    use flate2::read::GzDecoder;

    let mut decoder = GzDecoder::new(data);
    
    decoder.read_to_end(buf).unwrap()
}
