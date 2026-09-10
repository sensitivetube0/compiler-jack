
use std::error::Error;
use std::fs::{self, File, metadata};
use std::io::{self, BufReader};

pub fn handle_file_or_dir(path_name:&str) -> Result<Vec<BufReader<File>>,Box< dyn Error>>{


    let md = metadata(path_name)?;
  
    if md.is_file(){
       Ok(handle_file(path_name)?)
    }else if md.is_dir(){
        Ok(handle_dir(path_name)?)
    }else{
        Err("path is neither a directory or file".into())
    }

    

}


fn handle_file(path_name:&str) -> Result<Vec<BufReader<File>>,io::Error>{
    let file = File::open(path_name)?;
    let reader = BufReader::new(file);

    Ok(vec![reader])

}

fn handle_dir(path_name:&str) -> Result<Vec<BufReader<File>>,io::Error>{
    let paths = fs::read_dir(path_name)?;

    let mut files:Vec<BufReader<File>> = vec![];

    for path in paths{
        let entry = path?;

        if entry.file_type()?.is_file(){
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);
            files.push(reader);
        }
    }


    Ok(files)



}