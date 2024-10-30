use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use godot::classes::{Os, ProjectSettings};
use http::{Request, Response};
use http::header::CONTENT_TYPE;

pub fn get_res_response(
    request: Request<Vec<u8>>,
) -> Response<Cow<'static, [u8]>> {
    let os = Os::singleton();
    let root = if os.has_feature("editor".into()) {
        let project_settings = ProjectSettings::singleton();
        PathBuf::from(String::from(project_settings.globalize_path("res://".into())))
    } else {
        let mut dir = PathBuf::from(String::from(os.get_executable_path()));
        dir.pop();
        dir
    };

    let path = format!("{}{}", request.uri().host().unwrap_or_default(), request.uri().path());
    let full_path = root.join(path);
    if full_path.exists() && full_path.is_file() {
        let content = fs::read(full_path).expect("Failed to read file");
        let mime = infer::get(&content).expect("File type is unknown");
        return http::Response::builder()
            .header(CONTENT_TYPE, mime.to_string())
            .status(200)
            .body(content)
            .unwrap()
            .map(Into::into);
    }

    http::Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .status(404)
        .body(format!("Could not find file at {:?}", full_path).as_bytes().to_vec())
        .unwrap()
        .map(Into::into)
}