use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use ssh2::Session;
use std::env;
use std::fs::File;
use std::io;


/*
upload file to hardcoded sftp upload directory

TODO: create a class out of this
*/

fn main()
{
  let server = "127.0.0.1:22";

  let args: Vec<String> = env::args().collect();

  if args.len() < 2
  {
    println!("filename required");
    std::process::exit(1);
  }

  let upload_filename = &args[1];
  let upload_filename_only = Path::new( upload_filename ).file_name().unwrap().to_str().unwrap();

  let mut sess = Session::new().unwrap_or_else(|| {
    println!("sftp session failed");
    std::process::exit(1);
  });
  let tcp = TcpStream::connect(server).unwrap_or_else(|a| {
    println!("unable to connect {} - {}",server,a);
    std::process::exit(1);
  });

  sess.handshake(&tcp).unwrap();
  sess.userauth_password("sftpuser", "sftp").unwrap();

  let sftpsess = sess.sftp().unwrap();
  let mut file = sftpsess.create(Path::new( &format!("upload/{}", upload_filename_only) )).unwrap_or_else(|a| {
    println!("{}",a);
    std::process::exit(1);
  });

  let mut input_file = File::open( upload_filename ).unwrap_or_else(|a|{
    println!("{}", a);
    std::process::exit(1);
  });
  let metadata = std::fs::metadata( upload_filename );
  let ll = metadata.unwrap().len();
  let filesize:usize = ll as usize;

  let mut buffer = [0; 1024*8];
  let mut written:usize = 0;

  while written < filesize 
  {
    let g = input_file.read(&mut buffer); 
    let read:usize = file.write(&buffer[0..g.unwrap() as usize]).unwrap_or_else(|a|{
      println!("error - {}",a);
      std::process::exit(1);
    }).into();
    written += read;
  }

  println!("done");
}
