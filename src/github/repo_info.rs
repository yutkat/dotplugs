use super::GitHubInfo;
use crate::repository::Repository;
use anyhow::{anyhow, Context, Result};
use graphql_client::*;
use log::*;
use serde::*;

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

pub fn create_info(repos: &Vec<Repository>) -> Result<Vec<GitHubInfo>> {
    let search_query = convert_query(repos)?;
    let mut github_info: Vec<GitHubInfo> = vec![];
    let mut next_cursor: Option<String> = None;
    loop {
        let response_data = download_repository_info(&search_query, &next_cursor)?;
        github_info.extend(convert_github_info(&response_data)?);
        next_cursor = continue_search(&response_data)?;
        if next_cursor.is_none() {
            break;
        }
    }
    debug!("{:?}", github_info);
    Ok(github_info)
}

fn convert_query(repos: &Vec<Repository>) -> Result<String> {
    let query: String = repos
        .iter()
        .map(|r| r.get_name_with_owner())
        .filter_map(Result::ok)
        .map(|r| format!(" repo:{}", r))
        .collect();
    Ok(query)
}

fn download_repository_info<S: Into<String>>(
    search_query: S,
    after: &Option<String>,
) -> Result<repo_view::ResponseData> {
    dotenv::dotenv().ok();
    let config: Env = envy::from_env().context("while reading from environment")?;

    let q = RepoView::build_query(repo_view::Variables {
        query: search_query.into(),
        after: after.as_ref().cloned(),
    });

    let client = reqwest::Client::new();

    let mut res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_api_token)
        .json(&q)
        .send()?;

    let response_body: Response<repo_view::ResponseData> = res.json()?;
    debug!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");
        for error in &errors {
            error!("{:?}", error);
        }
        return Err(anyhow!("GraphQL error"));
    }

    let response_data: repo_view::ResponseData =
        response_body.data.ok_or(anyhow!("missing response data"))?;
    Ok(response_data)
}

fn convert_github_info(response_data: &repo_view::ResponseData) -> Result<Vec<GitHubInfo>> {
    let repos = response_data
        .search
        .nodes
        .as_ref()
        .ok_or(anyhow!("missing"))?;
    let github_info: Vec<GitHubInfo> = repos
        .into_iter()
        .map(|repo| match repo.as_ref().unwrap() {
            repo_view::RepoViewSearchNodes::Repository(r) => Some(GitHubInfo {
                name_with_owner: r.name_with_owner.to_string(),
                stargazers: r.stargazers.total_count,
            }),
            _ => None,
        })
        .filter_map(|v| v)
        .collect();
    Ok(github_info)
}

fn continue_search(response_data: &repo_view::ResponseData) -> Result<Option<String>> {
    let has_next_page: bool = response_data.search.page_info.has_next_page;
    if !has_next_page {
        return Ok(None);
    }
    let next_cursor = response_data.search.page_info.end_cursor.clone();
    Ok(next_cursor)
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
    fn create_info_works() -> Result<()> {
        init();

        let v = vec![
            Repository {
                uri: "https://git::@github.com/kana/vim-operator-user.git".to_string(),
                dir: "/home/test/.vim/plugged/vim-operator-user/".to_string(),
            },
            Repository {
                uri: "https://git::@github.com/moll/vim-bbye.git".to_string(),
                dir: "/home/test/.vim/plugged/vim-bbye/".to_string(),
            },
        ];

        let _ = create_info(&v)?;
        Ok(())
    }
}
