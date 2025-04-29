use std::env;
use std::fs;
use std::time::SystemTime;
use std::io::Write;

fn main() {
    let args: Vec<String> =  env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path);
    match contents{
        Ok(content) => {
            create_backup_file(&format!("{}", file_path), &content);
            println!("Content read successfully");
        },
        Err(_) => {
            println!("Problem reading file");
            return;
        }
    }    
}

fn create_backup_file(file_path: &String, content: &String){
    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    match fs::File::create(format!("{}.{}.bak", file_path, timestamp.to_string())) {
        Ok(mut file) => {
            let _ = file.write_all(content.as_bytes());
        },
        Err(_) => panic!("File Couldnt be created")
    }
}