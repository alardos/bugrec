use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use http::http_request::HttpRequest;
use http::http_response::HttpResponse;

use crate::AppState;

pub fn resolve_img(request: &HttpRequest, app_state: Arc<AppState>) -> HttpResponse {
    let Some(path) = request.get_param::<String>("path") else {
        return HttpResponse::bad_from("missing param: path")
    };

    let full_path: PathBuf = PathBuf::from(format!("resources/{path}"));
    return match fs::File::open(full_path) {
        Ok(mut f) => {
            let mut buffer = vec![];
            _=f.read_to_end(&mut buffer);
            HttpResponse::ok_from(buffer)
        },
        Err(_) => HttpResponse::not_found()
    }
} 

