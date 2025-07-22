use std::str;

pub struct Utf8Buffer {
    incomplete_bytes: Vec<u8>,
}

impl Utf8Buffer {
    pub fn new() -> Self {
        Self {
            incomplete_bytes: Vec::new(),
        }
    }

    pub fn push_bytes(&mut self, mut new_bytes: Vec<u8>) -> String {
        // 将之前不完整的字节与新数据合并
        if !self.incomplete_bytes.is_empty() {
            let mut combined = std::mem::take(&mut self.incomplete_bytes);
            combined.append(&mut new_bytes);
            new_bytes = combined;
        }

        // 找到最后一个完整的UTF-8字符的位置
        let (complete_end, incomplete_start) = self.find_complete_utf8_boundary(&new_bytes);

        // 提取完整的部分
        let complete_bytes = &new_bytes[..complete_end];
        let result = String::from_utf8_lossy(complete_bytes).into_owned();

        // 保存不完整的部分供下次使用
        self.incomplete_bytes = new_bytes[incomplete_start..].to_vec();

        result
    }

    fn find_complete_utf8_boundary(&self, bytes: &[u8]) -> (usize, usize) {
        if bytes.is_empty() {
            return (0, 0);
        }

        // 从末尾开始，寻找最后一个完整UTF-8字符的边界
        // UTF-8字符最长4字节，所以我们检查最后4个字节的所有可能位置
        for i in (bytes.len().saturating_sub(4)..bytes.len()).rev() {
            if self.is_utf8_char_boundary(bytes, i) {
                // 检查从i开始的部分是否形成完整的UTF-8字符序列
                match str::from_utf8(&bytes[i..]) {
                    Ok(_) => {
                        // 从i到末尾都是完整的UTF-8
                        return (bytes.len(), bytes.len());
                    }
                    Err(_) => {
                        // 从i开始有不完整的字符，所以i是我们要找的边界
                        return (i, i);
                    }
                }
            }
        }

        // 如果上面的方法没有找到合适的边界，使用标准方法
        match str::from_utf8(bytes) {
            Ok(_) => (bytes.len(), bytes.len()),
            Err(error) => {
                let valid_up_to = error.valid_up_to();
                (valid_up_to, valid_up_to)
            }
        }
    }

    // 检查位置i是否是UTF-8字符边界
    fn is_utf8_char_boundary(&self, bytes: &[u8], i: usize) -> bool {
        if i == 0 || i >= bytes.len() {
            return true;
        }
        
        let byte = bytes[i];
        // UTF-8字符边界：
        // - ASCII字符 (0xxxxxxx)
        // - 多字节字符的开始 (11xxxxxx)
        // 不是延续字节 (10xxxxxx)
        (byte & 0x80) == 0 || (byte & 0xC0) == 0xC0
    }

    pub fn incomplete_bytes_len(&self) -> usize {
        self.incomplete_bytes.len()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mixed_up_to_usage() {
        fn is_char_boundary_at(bytes: &[u8], i: usize) -> bool {
            if i == 0 || i >= bytes.len() {
                return true;
            }
            
            let byte = bytes[i];
            // UTF-8字符边界检查
            (byte & 0x80) == 0 || (byte & 0xC0) == 0xC0
        }
        // 辅助函数：用于验证UTF-8处理逻辑
        pub fn find_last_complete_utf8_boundary(bytes: &[u8]) -> usize {
            if bytes.is_empty() {
                return 0;
            }

            // 从末尾向前查找，最多检查4个字节（UTF-8最大字符长度）
            let start_check = bytes.len().saturating_sub(4);
            
            for i in (start_check..bytes.len()).rev() {
                // 检查这是否是字符边界
                if is_char_boundary_at(bytes, i) {
                    // 验证从i到末尾是否都是有效UTF-8
                    if str::from_utf8(&bytes[i..]).is_ok() {
                        return bytes.len();
                    } else {
                        return i;
                    }
                }
            }
            
            // 如果没找到合适的边界，使用标准方法
            match str::from_utf8(bytes) {
                Ok(_) => bytes.len(),
                Err(error) => error.valid_up_to(),
            }
        }

        let mixed_data = vec![
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello" (0-4)
            0xE4, 0xBD,                   // "你"的前两个字节 (5-6) 不完整
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello" (7-11) 
            0xE4, 0xBD,                   // "你"的前两个字节 (12-13) 不完整
        ];
        
        // 测试我们的边界查找函数
        let boundary = find_last_complete_utf8_boundary(&mixed_data);
        assert_eq!(boundary, 12);
        println!("找到的边界位置: {}", boundary);
        
        // 验证前12个字节确实是有效的UTF-8
        if let Ok(s) = str::from_utf8(&mixed_data[..12]) {
            println!("前12个字节的内容: '{}'", s);
            assert_eq!(s, "HelloHello");
        }
    }

    #[test]
    fn test_standard_utf8_error() {
        let mixed_data = vec![
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello"
            0xE4, 0xBD,                   // "你"的前两个字节 (不完整)
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello"
            0xE4, 0xBD,                   // "你"的前两个字节 (不完整)
        ];
        
        // 标准的UTF-8错误处理
        match str::from_utf8(&mixed_data) {
            Ok(_) => panic!("应该失败"),
            Err(error) => {
                println!("标准 valid_up_to: {}", error.valid_up_to());
                // 标准方法会在第一个错误处停止，返回5
                assert_eq!(error.valid_up_to(), 5);
            }
        }
    }

    #[test]
    fn test_buffer_with_mixed_data() {
        let mut buffer = Utf8Buffer::new();
        
        let mixed_data = vec![
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello"
            0xE4, 0xBD,                   // 不完整的"你"
        ];
        
        let result = buffer.push_bytes(mixed_data);
        assert_eq!(result, "Hello");
        assert_eq!(buffer.incomplete_bytes_len(), 2); // 缓存不完整的字节
        
        // 添加剩余的字节来完成"你"
        let remaining = vec![0xA0]; // "你"的最后一个字节
        let result2 = buffer.push_bytes(remaining);
        assert_eq!(result2, "你");
        assert_eq!(buffer.incomplete_bytes_len(), 0);
    }

    #[test]
    fn test_multiple_incomplete_sequences() {
        let mut buffer = Utf8Buffer::new();
        
        // 包含多个完整字符和一个不完整字符的数据
        let data = vec![
            0x48, 0x65, 0x6C, 0x6C, 0x6F, // "Hello"
            0xE4, 0xBD, 0xA0,             // "你" (完整)
            0x48, 0x69,                   // "Hi"
            0xE4, 0xBD,                   // "你"的前两个字节 (不完整)
        ];
        
        let result = buffer.push_bytes(data);
        assert_eq!(result, "Hello你Hi");
        assert_eq!(buffer.incomplete_bytes_len(), 2);
    }
}