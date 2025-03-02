use std::fs;
//use std::path::Path;

fn main() -> std::io::Result<()>{
    let entries = fs::read_dir("./")?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            println!("{:?}", file_name);
        }else{
            println!("No file name");
        }
    }
    Ok(())
}