#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rand;

use std::io;
use std::fs::File;
use std::path::Path;

use rocket::Data;
use rocket::response::{content, Debug};

use meta_srv::paste_id::PasteID;

const HOST: &str = "http://localhost:8000";
const ID_LENGTH: usize = 3;

/// pastebin是一个简单的Web应用程序，允许用户上载文本文档，以后再通过特殊URL对其进行检索。
/// 它们通常用于共享代码段，配置文件和错误日志。
/// 一个简单的pastebin服务，该服务允许用户从其终端上载文件。该服务将使用上载文件的URL进行回复。

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, Debug<io::Error>> {
    let id = PasteID::new(ID_LENGTH);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = HOST, id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<content::Plain<File>> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).map(|f| content::Plain(f)).ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXMAPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, upload, retrieve])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    use super::{rocket, index};
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};

    fn extract_id(from: &str) -> Option<String> {
        from.rfind('/').map(|i| &from[(i + 1)..]).map(|s| s.trim_end().to_string())
    }

    #[test]
    fn check_index() {
        let client = Client::new(rocket()).unwrap();

        // Ensure the index returns what we expect.
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        assert_eq!(response.body_string(), Some(index().into()))
    }

    fn upload_paste(client: &Client, body: &str) -> String {
        let mut response = client.post("/").body(body).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::Plain));
        extract_id(&response.body_string().unwrap()).unwrap()
    }

    fn download_paste(client: &Client, id: &str) -> String {
        let mut response = client.get(format!("/{}", id)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        response.body_string().unwrap()
    }

    #[test]
    fn pasting() {
        let client = Client::new(rocket()).unwrap();

        // Do a trivial upload, just to make sure it works.
        let body_1 = "Hello, world!";
        let id_1 = upload_paste(&client, body_1);
        assert_eq!(download_paste(&client, &id_1), body_1);

        // Make sure we can keep getting that paste.
        assert_eq!(download_paste(&client, &id_1), body_1);
        assert_eq!(download_paste(&client, &id_1), body_1);
        assert_eq!(download_paste(&client, &id_1), body_1);

        // Upload some unicode.
        let body_2 = "こんにちは";
        let id_2 = upload_paste(&client, body_2);
        assert_eq!(download_paste(&client, &id_2), body_2);

        // Make sure we can get both pastes.
        assert_eq!(download_paste(&client, &id_1), body_1);
        assert_eq!(download_paste(&client, &id_2), body_2);
        assert_eq!(download_paste(&client, &id_1), body_1);
        assert_eq!(download_paste(&client, &id_2), body_2);

        // Now a longer upload.
        let body_3 = "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed
            do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim
            ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
            aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit
            in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
            Excepteur sint occaecat cupidatat non proident, sunt in culpa qui
            officia deserunt mollit anim id est laborum.";
        let id_3 = upload_paste(&client, body_3);
        assert_eq!(download_paste(&client, &id_3), body_3);
        assert_eq!(download_paste(&client, &id_1), body_1);
        assert_eq!(download_paste(&client, &id_2), body_2);
    }

}

