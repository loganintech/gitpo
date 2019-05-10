pub mod bitbucket;
pub mod github;
pub mod gitlab;

pub trait Provider {
    fn payload(&self) -> String;
    fn endpoint(&self) -> String;
    fn extract_url(&self, src: &reqwest::header::HeaderMap) -> String;
    fn token(&self) -> String;
    fn auth_header(&self) -> String;
}
