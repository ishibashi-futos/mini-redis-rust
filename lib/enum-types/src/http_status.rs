// 整数値を指定することもできる
pub enum HttpStatusCode {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
    IamaTeapot = 418,
    InternalError = 500,
}

impl HttpStatusCode {
    pub fn from_u32(n: u32) -> Option<HttpStatusCode> {
        match n {
            200 => Some(HttpStatusCode::Ok),
            304 => Some(HttpStatusCode::NotModified),
            404 => Some(HttpStatusCode::NotFound),
            418 => Some(HttpStatusCode::IamaTeapot),
            500 => Some(HttpStatusCode::InternalError),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::HttpStatusCode;

    #[test]
    fn cast_status_code_ok() {
        assert_eq!(200, HttpStatusCode::Ok as i32);
    }

    #[test]
    fn cast_status_code_not_modified() {
        assert_eq!(304, HttpStatusCode::NotModified as i32);
    }

    #[test]
    fn cast_status_code_not_found() {
        assert_eq!(404, HttpStatusCode::NotFound as i32);
    }

    #[test]
    fn cast_status_code_teapot() {
        assert_eq!(418, HttpStatusCode::IamaTeapot as i32);
    }

    #[test]
    fn cast_status_code_internal_error() {
        assert_eq!(500, HttpStatusCode::InternalError as i32);
    }
}
