use godot::builtin::GString;
use godot::classes::file_access::ModeFlags;
use godot::classes::FileAccess;
use http::{Request, Response};
use http::header::{ACCEPT_RANGES, CONTENT_RANGE, CONTENT_TYPE, RANGE};
use lazy_static::lazy_static;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn get_res_response(request: Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let root = PathBuf::from("res://");
    let path = format!(
        "{}{}",
        request.uri().host().unwrap_or_default(),
        request.uri().path()
    );
    let mut full_path = root.join(path);

    if full_path.ends_with("/") || full_path.is_dir() || full_path.extension().is_none() {
        let index_path = full_path.join("index.html");
        
        if FileAccess::file_exists(&GString::from(index_path.to_str().unwrap_or_default())) {
            full_path = index_path;
        }
    }

    let full_path_str = GString::from(full_path.to_str().unwrap_or_default());

    if !FileAccess::file_exists(&full_path_str) {
        return http::Response::builder()
            .header(CONTENT_TYPE, "text/plain")
            .status(404)
            .body(Cow::from(
                format!("Could not find file at {:?}", full_path)
                    .as_bytes()
                    .to_vec(),
            ))
            .expect("Failed to build 404 response");
    }

    let extension = full_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

    let content_type = MIME_TYPES
            .get(extension)
            .unwrap_or(&"application/octet-stream");

    return FileAccess::open(&full_path_str, ModeFlags::READ)
        .map(|mut file| {
            let file_size: u64 = file.get_length().try_into().expect("failed to get file size");

            // The client might request a file with Range,
            // even if we set Accept-Ranges to none, Safari does this while loading media types.
            // So, we MUST implement the Content-Range logic to serve the file correctly.
            let mut content_range: Option<(u64, u64)> = None;
            if let Some(range) = request.headers().get(RANGE) {
                let range_str = range.to_str().expect("failed to parse Range header");

                // the range header might be in the format "bytes=start-end", "bytes=start-", or "bytes=-end"
                let parts: Vec<&str> = range_str[6..].split('-').collect();
                let start = parts[0].parse::<u64>().unwrap_or(0);
                let end = parts[1].parse::<u64>().unwrap_or(file_size - 1);
                content_range = Some((start, end));
            }

            return if let Some((start, end)) = content_range {
                if start >= file_size {
                    return http::Response::builder()
                        .header(CONTENT_TYPE, *content_type)
                        .header(ACCEPT_RANGES, "bytes")
                        .header(CONTENT_RANGE, format!("bytes */{}", file_size))
                        .status(416) // Range Not Satisfiable
                        .body(Cow::from(Vec::new()))
                        .expect("Failed to build 416 response");
                }

                let end = if end == 0 || end >= file_size {
                    file_size - 1
                } else {
                    end
                };

                let content_size = (end - start + 1) as i64;
                file.seek(start);
                let content = file.get_buffer(content_size).as_slice().to_vec();

                http::Response::builder()
                    .header(CONTENT_TYPE, *content_type)
                    .header(ACCEPT_RANGES, "bytes")
                    .header(CONTENT_RANGE, format!("bytes {}-{}/{}", start, end, file_size))
                    .status(206)
                    .body(Cow::from(content))
                    .expect("Failed to build 206 response")
            } else {
                let content_size = file_size as i64;
                let content = file.get_buffer(content_size).as_slice().to_vec();
                http::Response::builder()
                    .header(CONTENT_TYPE, *content_type)
                    .status(200)
                    .body(Cow::from(content))
                    .expect("Failed to build 200 response")
            }
        }).unwrap_or_else(|| {
            http::Response::builder()
                .header(CONTENT_TYPE, "text/plain")
                .status(500)
                .body(Cow::from(
                    format!("Failed to open at {:?}", full_path)
                        .as_bytes()
                        .to_vec(),
                ))
                .expect("Failed to build 404 response")
        });
}

lazy_static! {
    static ref MIME_TYPES: HashMap<&'static str, &'static str> = HashMap::from([
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/MIME_types/Common_types
        ("aac", "audio/aac"),
        ("abw", "application/x-abiword"),
        ("apng", "image/apng"),
        ("arc", "application/x-freearc"),
        ("avif", "image/avif"),
        ("avi", "video/x-msvideo"),
        ("azw", "application/vnd.amazon.ebook"),
        ("bin", "application/octet-stream"),
        ("bmp", "image/bmp"),
        ("bz", "application/x-bzip"),
        ("bz2", "application/x-bzip2"),
        ("cda", "application/x-cdf"),
        ("cjs", "text/javascript"),
        ("csh", "application/x-csh"),
        ("css", "text/css"),
        ("csv", "text/csv"),
        ("doc", "application/msword"),
        ("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
        ("eot", "application/vnd.ms-fontobject"),
        ("epub", "application/epub+zip"),
        ("gz", "application/gzip"),
        ("gif", "image/gif"),
        ("html", "text/html"),
        ("htm", "text/html"),
        ("ico", "image/vnd.microsoft.icon"),
        ("ics", "text/calendar"),
        ("jar", "application/java-archive"),
        ("jpeg", "image/jpeg"),
        ("jpg", "image/jpeg"),
        ("js", "text/javascript"),
        ("json", "application/json"),
        ("jsonld", "application/ld+json"),
        ("midi", "audio/midi"),
        ("mid", "audio/midi"),
        ("mjs", "text/javascript"),
        ("mp3", "audio/mpeg"),
        ("mp4", "video/mp4"),
        ("mpeg", "video/mpeg"),
        ("mpkg", "application/vnd.apple.installer+xml"),
        ("odp", "application/vnd.oasis.opendocument.presentation"),
        ("ods", "application/vnd.oasis.opendocument.spreadsheet"),
        ("odt", "application/vnd.oasis.opendocument.text"),
        ("oga", "audio/ogg"),
        ("ogv", "video/ogg"),
        ("ogx", "application/ogg"),
        ("opus", "audio/ogg"),
        ("otf", "font/otf"),
        ("png", "image/png"),
        ("pdf", "application/pdf"),
        ("php", "application/x-httpd-php"),
        ("ppt", "application/vnd.ms-powerpoint"),
        ("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation"),
        ("rar", "application/vnd.rar"),
        ("rtf", "application/rtf"),
        ("sh", "application/x-sh"),
        ("svg", "image/svg+xml"),
        ("tar", "application/x-tar"),
        ("tif", "image/tiff"),
        ("tiff", "image/tiff"),
        ("ts", "video/mp2t"),
        ("ttf", "font/ttf"),
        ("txt", "text/plain"),
        ("vsd", "application/vnd.visio"),
        ("wav", "audio/wav"),
        ("weba", "audio/webm"),
        ("webm", "video/webm"),
        ("webp", "image/webp"),
        ("woff", "font/woff"),
        ("woff2", "font/woff2"),
        ("xhtml", "application/xhtml+xml"),
        ("xls", "application/vnd.ms-excel"),
        ("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
        ("xml", "application/xml"),
        ("xul", "application/vnd.mozilla.xul+xml"),
        ("zip", "application/zip"),
        ("3gp", "video/3gpp"),
        ("3g2", "video/3gpp2"),
        ("7z", "application/x-7z-compressed"),
    ]);
}
