use std::sync::OnceLock;

use cafebabe::{parse_class_with_options, ClassFile, ParseOptions};

use eyre::Result;

#[derive(Eq, PartialEq)]
pub enum Parse {
    WithBytecode,
    WithoutBytecode,
}

/// This structure contains the raw bytes for the class file as well as a cache for the parsed form
pub struct ClassBlob<'a> {
    pub bytes: Vec<u8>,
    parse_mode: Parse,
    class: OnceLock<ClassFile<'a>>,
}

impl<'a> ClassBlob<'a> {
    /// Retrieve the parsed instance of the class backed by this blob
    pub fn class(&'a mut self) -> Result<&'a ClassFile<'a>> {
        let class = self.class.get_or_init(|| {
            let mut parse_options = ParseOptions::default();
            parse_options.parse_bytecode(self.parse_mode == Parse::WithBytecode);

            parse_class_with_options(&self.bytes, &parse_options).unwrap()
        });

        Ok(class)
    }
}

/// Reads the provided filename and returns the ClassBlob wrapper for class file bytes within
pub fn read_classfile(filename: &str, parse_mode: Parse) -> Result<ClassBlob> {
    let class_file_bytes = std::fs::read(filename)?;

    Ok(ClassBlob {
        bytes: class_file_bytes,
        parse_mode,
        class: OnceLock::new(),
    })
}
