const INFORMATIONAL: [i16; 4] = [100, 101, 102, 103];
const SUCCESS: [i16; 10] = [200, 201, 202, 203, 204, 205, 206, 207, 208, 226];
const REDIRECTION: [i16; 9] = [300, 301, 302, 303, 304, 305, 306, 307, 308];
const CLIENT_ERROR: [i16; 28] = [
    400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418,
    421, 422, 423, 424, 426, 428, 429, 431, 451,
];
const SERVER_ERROR: [i16; 11] = [500, 501, 502, 503, 504, 505, 506, 507, 508, 510, 511];

pub struct StatusCodes {}
impl StatusCodes {
    pub fn is_valid(status: i16) -> bool {
        if INFORMATIONAL.contains(&status) {
            return true;
        } else if SUCCESS.contains(&status) {
            return true;
        } else if REDIRECTION.contains(&status) {
            return true;
        } else if CLIENT_ERROR.contains(&status) {
            return true;
        } else if SERVER_ERROR.contains(&status) {
            return true;
        } else {
            return false;
        }
    }

    pub fn get_description(status: i16) -> String {
        match status {
            100 => "Continue".to_string(),
            101 => "Switching Protocols".to_string(),
            102 => "Processing".to_string(),
            103 => "Early Hints".to_string(),
            200 => "OK".to_string(),
            201 => "Created".to_string(),
            202 => "Accepted".to_string(),
            203 => "Non-Authoritative Information".to_string(),
            204 => "No Content".to_string(),
            205 => "Reset Content".to_string(),
            206 => "Partial Content".to_string(),
            207 => "Multi-Status".to_string(),
            208 => "Already Reported".to_string(),
            226 => "IM Used".to_string(),
            300 => "Multiple Choices".to_string(),
            301 => "Moved Permanently".to_string(),
            302 => "Found".to_string(),
            303 => "See Other".to_string(),
            304 => "Not Modified".to_string(),
            305 => "Use Proxy".to_string(),
            306 => "Switch Proxy".to_string(),
            307 => "Temporary Redirect".to_string(),
            308 => "Permanent Redirect".to_string(),
            400 => "Bad Request".to_string(),
            401 => "Unauthorized".to_string(),
            402 => "Payment Required".to_string(),
            403 => "Forbidden".to_string(),
            404 => "Not Found".to_string(),
            405 => "Method Not Allowed".to_string(),
            406 => "Not Acceptable".to_string(),
            407 => "Proxy Authentication Required".to_string(),
            408 => "Request Timeout".to_string(),
            409 => "Conflict".to_string(),
            410 => "Gone".to_string(),
            411 => "Length Required".to_string(),
            412 => "Precondition Failed".to_string(),
            413 => "Payload Too Large".to_string(),
            414 => "URI Too Long".to_string(),
            415 => "Unsupported Media Type".to_string(),
            416 => "Range Not Satisfiable".to_string(),
            417 => "Expectation Failed".to_string(),
            418 => "I'm A Teapot".to_string(),
            421 => "Misdirected Request".to_string(),
            422 => "Unprocessable Entity".to_string(),
            423 => "Locked".to_string(),
            424 => "Failed Dependency".to_string(),
            425 => "Too Early".to_string(),
            426 => "Upgrade Required".to_string(),
            428 => "Precondition Required".to_string(),
            429 => "Too Many Requests".to_string(),
            431 => "Requests Headers Fields Too Large".to_string(),
            451 => "Unavailable For Legal Reasons".to_string(),
            500 => "Internal Server Error".to_string(),
            501 => "Not Implemented".to_string(),
            502 => "Bad Gateway".to_string(),
            503 => "Service Unavailable".to_string(),
            504 => "Gateway Timeout".to_string(),
            505 => "HTTP Version Not Supported".to_string(),
            506 => "Variant Also Negotiates".to_string(),
            507 => "Insufficient Storage".to_string(),
            508 => "Loop Detected".to_string(),
            510 => "Not Extended".to_string(),
            511 => "Network Authentication Required".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn get_explanation(_status: i16) -> String {
        "Coming soon. Not implemented yet. Join @ Schrodinger Hat to help us develop this feature!".to_string()
    }
}
