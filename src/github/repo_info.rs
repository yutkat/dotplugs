use super::GitHubInfo;
use failure::Error;
use failure::*;
use graphql_client::*;
use log::*;
use serde::*;

type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/github/graphql/schema.graphql",
    query_path = "src/github/graphql/repo_info_query.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

fn create_info() -> Result<Vec<GitHubInfo>, Error> {
    let search_query = "repo:yutakatay/dotfiles";
    let response_data = download_repository_info(search_query)?;
    let github_info = convert_github_info(response_data)?;
    Ok(github_info)
}

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), failure::Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

fn download_repository_info<S: Into<String>>(
    search_query: S,
) -> Result<repo_view::ResponseData, Error> {
    dotenv::dotenv().ok();
    let config: Env = envy::from_env().context("while reading from environment")?;

    let q = RepoView::build_query(repo_view::Variables {
        query: search_query.into().to_string(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&q)
        .send()?;

    let response_body: Response<repo_view::ResponseData> = res.json()?;
    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");
        for error in &errors {
            error!("{:?}", error);
        }
        return Err(format_err!("GraphQL error"));
    }

    let response_data: repo_view::ResponseData = response_body
        .data
        .ok_or(format_err!("missing response data"))?;
    return Ok(response_data);
}

fn convert_github_info(response_data: repo_view::ResponseData) -> Result<Vec<GitHubInfo>, Error> {
    let repos = response_data.search.nodes.ok_or(format_err!("missing"))?;
    let mut github_info = vec![];
    let repo: &repo_view::RepoViewSearchNodes = repos.get(0).unwrap().as_ref().unwrap();
    match repo {
        repo_view::RepoViewSearchNodes::Repository(r) => {
            let info = GitHubInfo {
                uri: "".to_string(),
                dir: "".to_string(),
                stargazers: r.stargazers.total_count,
            };
            println!("{:?}", info);
            github_info.push(info);
        }
        _ => {}
    }
    Ok(github_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn create_info_works() -> Result<(), Error> {
        init();
        let _ = create_info()?;
        Ok(())
    }
}
