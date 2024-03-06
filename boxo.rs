use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;


fn main() {
    let mut cmd_line:std::env::Args = std::env::args();
    if cmd_line.len() < 2 {
        println!(" ");
        println!("boxoIO CLI");
        println!("type -help for command list");
    }
    else {
    cmd_line.next().unwrap();
    
    
    let ftype = cmd_line.next().unwrap();


    match ftype.as_str() {
    
        "read"=> {
            let source = cmd_line.next().unwrap();
            let mut contents = String::new();
            let mut file1 = std::fs::File::open(source).unwrap();
            file1.read_to_string(&mut contents).unwrap();
            print!("{}",contents)
        },
    
        "write"=> {
            let source = cmd_line.next().unwrap();
            let d = cmd_line.next().unwrap();
            let mut file2 = OpenOptions::new().append(true).open(source).expect("no beuno :(");
            file2.write_all(d.as_bytes()).expect("no beuno :(");
            file2.write_all(("\n").as_bytes()).expect("no beuno :(");

        },
    
        "create"=> {
            let source = cmd_line.next().unwrap();
            let _= std::fs::File::create(source).unwrap();
        },

        "del"=> {
            let source = cmd_line.next().unwrap();
            let _= std::fs::remove_file(source).unwrap();
        },
        "delete"=> {
            let source = cmd_line.next().unwrap();
            let _= std::fs::remove_file(source).unwrap();
        },

        "copy"=> {
            let source = cmd_line.next().unwrap();
            let new_source = cmd_line.next().unwrap();

            let mut file3 = std::fs::File::open(source).unwrap();
            let mut contents = String::new();
            file3.read_to_string(&mut contents).unwrap();
            
            let mut new_file = std::fs::File::create(new_source).unwrap();
            new_file.write(contents.as_bytes()).expect("no beuno :(");
        
        }
        
        "-help"=> {
            println!(" ");
            println!("boxoIO CLI");
            print!("    [read] - params [filename]: reads a file. \n    [write] - params [filename, text]: appends a line of text to a file \n    [create] - params [filename]: creates a file \n    [del] - params [filename]: deletes a file \n    [copy] - params [filename, new_filename]: copies the contents of a file to another file \n");
        }
        
        &_ => println!("ftype does not support this argument. please use -help for more info"),
        
    }
    }
}