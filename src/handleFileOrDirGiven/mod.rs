
use std::fs::{metadata,self};
use std::io::self;

pub fn handleFileOrDirGivenFun(path_name:&str) -> Result<(),io::Error>{


    let md = metadata(path_name)?;
    let isFile = md.is_file();
    Ok(())
}


fn handleFile(){




}