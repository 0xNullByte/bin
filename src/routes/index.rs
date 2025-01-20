use rocket_dyn_templates::Template;
use std::time::SystemTime;

use std::collections::HashMap;

use crate::get_parsed_args;
use crate::models::response_wrapper::ResponseWrapper;

#[get("/")]
pub async fn index() -> ResponseWrapper<Template> {
    let mut map = HashMap::new();

    // whether to include `/client` info
    let client_desc = match get_parsed_args().client_desc {
        true => "placeholder",
        false => "",
    };

    // List of files inside upload direcotry.
    let mut entries = match std::fs::read_dir(get_parsed_args().upload) {
        Ok(entries) => {
            entries.filter_map(|entry| entry.ok()).collect::<Vec<_>>()
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            vec![]
        }
    };

    // Sort entries by modification time.
    entries.sort_by(|a, b| {
        let a_time = a
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let b_time = b
            .metadata()
            .and_then(|meta| meta.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        b_time.cmp(&a_time)
    });

    // Joining the file names into a single string separated by spaces " " as the delimiter.
    let files = entries
        .iter()
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
        .collect::<Vec<_>>()
        .join(" ");

    map.insert("title", "bin");
    map.insert("client_desc", client_desc);
    map.insert("files", &files);

    ResponseWrapper::meta_response(Template::render("index.html", &map))
}
