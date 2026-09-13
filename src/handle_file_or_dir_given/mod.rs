
use std::error::Error;
use std::fs::{self, metadata};

use std::io::{self};
use std::path::{Path, PathBuf};

pub struct FilesInPathBuf{
    paths:Vec<PathBuf>,
}

impl FilesInPathBuf{

    pub fn get_index_in_paths(&self,index:usize) -> &PathBuf{
        if index > self.paths.len() - 1{
            panic!("Error index out of bounds please provide a valid input")
        }
        &self.paths[index]
    }
    pub fn len_of(&self) -> usize{
        self.paths.len()
    }
}




pub fn handle_file_or_dir<P: AsRef<Path>>(path_name:P) -> Result<FilesInPathBuf,Box< dyn Error>>{


    let md = metadata(&path_name)?;
    let path = path_name.as_ref();


    if md.is_file(){
       Ok(
        FilesInPathBuf { paths:vec![path.into()]  }
       )
    }else if md.is_dir(){
        let result_of_files_from_dir  = handle_dir(path)?;
        Ok(result_of_files_from_dir)
    }else{
        Err("path is neither a directory or file".into())
    }
    
    

}



fn handle_dir(path_name:&Path) -> Result<FilesInPathBuf,io::Error>{
    let paths = fs::read_dir(path_name)?;

    let mut paths_buf_files:Vec<PathBuf> = vec![];

    for path in paths{
        let entry = path?;
  
        if entry.file_type()?.is_file(){

            paths_buf_files.push(entry.path());
        }
    }


    Ok(
        FilesInPathBuf { paths:paths_buf_files }
    )



}