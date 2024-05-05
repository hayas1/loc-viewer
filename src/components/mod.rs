pub mod background;
pub mod darkmode;
pub mod forms;
pub mod home;
pub mod routes;
pub mod statistics;

pub const REPOSITORY: &str = "https://github.com/hayas1/loc-viewer";
pub const BASENAME: &str = "/loc-viewer/"; // TODO do not hard code basename

pub const STORAGE_KEY_DARKMODE: &str = concat!(env!("CARGO_PKG_NAME"), "/cfg/darkmode");
