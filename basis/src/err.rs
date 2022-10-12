use std::process::Command;
//use std::io::ErrorKind;
use std::io::{Error, ErrorKind};

pub fn error_handling_example(dir:&str)-> Result<i32, Error>{
    println!("\n\n");
    let mut list_cmd = Command::new("ls");
    //list_cmd.current_dir(dir).status().expect("Failed to execute list command.");
    // match list_cmd.current_dir(dir).status(){
    //     Ok(cmd)=>cmd,
    //     Err(e)=>panic!("Error:{}", e),
    // };
    // match list_cmd.current_dir(dir).status(){
    //     Ok(cmd)=>Some(cmd),
    //     Err(e)=>{
    //         print!("Directory not found");
    //         None
    //     },
    // };   
    // match list_cmd.current_dir(dir).status(){
    //     Ok(cmd)=>Some(cmd),
    //     Err(e)=> match e.kind(){
    //         ErrorKind::NotFound=>{
    //             print!("Directory not found");
    //             None  
    //         },
    //         _=> panic!("An unexpected error has occured!")
    //     },
    // };
    // list_cmd.current_dir(dir).status().unwrap();
    list_cmd.current_dir(dir).status()?; //poweful shorthand


    println!("\n\n");
    Ok(1)
}
