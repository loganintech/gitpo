use super::Provider;
use serde::Serialize;
use serde_json::to_string;
use structopt::StructOpt;

#[derive(Serialize, StructOpt)]
pub struct BitbucketArgs {
    #[structopt(short = "n", long = "name", help = "The name of the new repository.")]
    name: String,
    #[structopt(
        short = "t",
        long = "token",
        help = "A personal access token. Alternatively read from BITBUCKET_REPO_TOKEN env variable",
        env = "BITBUCKET_REPO_TOKEN"
    )]
    #[serde(skip_serializing)]
    pub token: String,
    #[structopt(
        long = "username",
        help = "Your bitbucket username. Alternatively read from BITBUCKET_USERNAME env variable",
        env = "BITBUCKET_USERNAME"
    )]
    #[serde(skip_serializing)]
    username: String,
    #[structopt(
        short = "d",
        long = "description",
        help = "A short description of the repository."
    )]
    description: Option<String>,
    #[structopt(
        long = "homepage",
        help = "A URL with more information about the repository."
    )]
    website: Option<String>,
    #[structopt(
        short = "p",
        long = "private",
        help = "Sets whether or not the repository is private."
    )]
    is_private: Option<bool>,
    #[structopt(
        short = "w",
        long = "wiki",
        help = "Enables or disables wikis for this repo. Defaults to true."
    )]
    has_wiki: Option<bool>,
    #[structopt(
        short = "i",
        long = "issues",
        help = "Enable or disable issues for this repo. Defaults to true."
    )]
    has_issues: Option<bool>,
    #[structopt(
        long = "fork_policy",
        help = "Allow public forking of this repo.",
        possible_value = "allow_forks",
        possible_value = "no_public_forks",
        possible_value = "no_forks"
    )]
    fork_policy: Option<String>,
    #[structopt(
        long = "scm",
        help = "Control the underlying source control method.",
        possible_value = "hg",
        possible_value = "git"
    )]
    scm: Option<String>,
    #[structopt(
        long = "language",
        help = "Give bitbucket a hint about the programming language."
    )]
    language: Option<String>,
    #[serde(skip_serializing)]
    #[structopt(
        short = "e",
        long = "endpoint",
        help = "Allows redirection of requests to enterprise providers."
    )]
    custom_endpoint: Option<String>,
}

const ENDPOINT: &str = "https://api.bitbucket.org/2.0/repositories/{username}/{slug}";

impl Provider for BitbucketArgs {
    fn payload(&self) -> String {
        to_string(&self).unwrap()
    }

    fn endpoint(&self) -> String {
        if let Some(custom_endpoint) = &self.custom_endpoint {
            custom_endpoint.to_string()
        } else {
            ENDPOINT
                .replace("{username}", &self.username)
                .replace("{slug}", &self.name)
        }
    }

    fn extract_url(&self, headers: &reqwest::header::HeaderMap) -> String {
        unimplemented!()
    }

}
