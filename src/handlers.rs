extern crate iron;
extern crate params;
extern crate multipart;


use iron::prelude::*;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::process::Command;
use iron::status;
use self::multipart::server::{Multipart, Entries, SaveResult};

use hyper::header::{ContentDisposition, DispositionType, DispositionParam, Charset};

/*
 Welcome Handler
*/
pub fn welcome_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Welcome to Teddy, see ya !")))
}

/*
 PingHandler
*/
pub fn ping_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "pong")))
}

/*
 Download Handler
*/
pub fn download_handler(req: &mut Request) -> IronResult<Response> {
    use self::params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["path"]) {
        Some(&Value::String(ref path)) if Path::new(path).exists() && Path::new(path).is_file() => {
            let file = File::open(path).unwrap();
            let file_name = Path::new(path).file_name();
            let mut response = Response::with((iron::status::Ok, file));
            response.headers.set(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(
                    Charset::Iso_8859_1, // The character set for the bytes of the filename
                    None, // The optional language tag (see `language-tag` crate)
                    Vec::from(format!("{:?}", file_name.unwrap()).replace("\"", "")) // the actual bytes of the filename
                )]
            });;
            Ok(response)
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}


/*
 Upload Handler
*/
pub fn upload_handler(request: &mut Request) -> IronResult<Response> {

    // Getting a multipart reader wrapper
    match Multipart::from_request(request) {
        Ok(mut multipart) => {
            // Fetching all data and processing it.
            // save().temp() reads the request fully, parsing all fields and saving all files
            // in a new temporary directory under the OS temporary directory.
            match multipart.save().temp() {

                SaveResult::Full(entries) => process_entries(entries),
                SaveResult::Partial(entries, reason) => {
                    process_entries(entries.keep_partial())?;
                    Ok(Response::with((
                        status::BadRequest,
                        format!("error reading request: {}", reason.unwrap_err())
                    )))
                }
                SaveResult::Error(error) => Ok(Response::with((
                    status::BadRequest,
                    format!("error reading request: {}", error)
                ))),
            }
        }
        Err(_) => {
            Ok(Response::with((status::BadRequest, "The request is not multipart")))
        }
    }
}

/// Processes saved entries from multipart request.
/// Returns an OK response or an error.
fn process_entries(entries: Entries) -> IronResult<Response> {

    let mut destination = String::new();

    for (name, field) in entries.fields {
        println!("Field {:?}: {:?}", name, field);

        if name == "destination" {
           destination = field.to_string();
        }
    }

    for (name, files) in entries.files {
        println!("Field {:?} has {} files:", name, files.len());

        for file in files {
            let oldpath = format!("{:?}", file.path).replace("\"", "");
            let newdir = format!("{:?}", destination).replace("\"", "");
            let newpath = format!("{:?}/{:?}", destination, file.filename.unwrap()).replace("\"", "");

            println!("upload destination will be {}", newpath);

            match fs::create_dir(newdir) {
                Err(why) => println!("create_dir failed {:?}", why.kind()),
                Ok(_) => {},
            }

            println!("Uploaded file moved from {} to {}", oldpath, newpath);

            match fs::rename(oldpath, newpath ) {
                Err(why) => println!("rename failed {:?}", why.kind()),
                Ok(_) => {},
            }
        }
    }

    Ok(Response::with((status::Ok, "Multipart data is processed")))
}


/*
 Exec Handler
*/
pub fn exec_handler(req: &mut Request) -> IronResult<Response> {

    use self::params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    if let Some(command) = map.get("command") {
        if let Some(parameter) = map.get("parameter") {

            println!("command = {:?}, parameter = {:?}", command, parameter);
            let zcommand = format!("{:?}", command).replace("\"", "");
            let zparameter = format!("{:?}", parameter).replace("\"", "");
            Command::new(zcommand)
                .env("PATH", "/home/henri/Documents/Apps/jvms/java-1.8.0-sun-x64/bin:/home/henri/Documents/Apps/jvms/java-1.8.0-sun-x64/jre/bin:/home/henri/bin:/home/henri/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/games:/usr/local/games:/home/henri/Documents/Apps/gos/go-1.7/bin:/home/henri/bin:/opt/mssql-tools/bin")
                .arg(zparameter)
                .spawn()
                .expect("command failed to start");
            return Ok(Response::with((iron::status::Ok, "exec")));
        }
    }
    return Ok(Response::with((iron::status::BadRequest, "Specify command and parameter query params")));
}