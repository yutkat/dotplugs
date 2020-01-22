use super::GitHubInfo;
use failure::Error;
use failure::*;
use graphql_client::*;
use log::*;
use prettytable::*;
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

fn parse_repo_name(repo_name: &str) -> Result<(&str, &str), Error> {
    let mut parts = repo_name.split('/');
    match (parts.next(), parts.next()) {
        (Some(owner), Some(name)) => Ok((owner, name)),
        _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
    }
}

fn create_info() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let config: Env = envy::from_env().context("while reading from environment")?;

    let repo = "yutakatay/dotfiles";
    let (owner, name) = parse_repo_name(&repo)?;

    let q = RepoView::build_query(repo_view::Variables {
        owner: owner.to_string(),
        name: name.to_string(),
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
        return Err("GraphQL error");
    }

    let response_data: repo_view::ResponseData = response_body.data.expect("missing response data");

    let stars = response_data
        .repository
        .as_ref()
        .map(|repo| repo.stargazers.total_count)
        .unwrap_or(0);

    println!("{}/{} - 🌟 {}", owner, name, stars);

    let mut table = prettytable::Table::new();

    table.add_row(row!(b => "issue", "comments"));

    for issue in &response_data
        .repository
        .expect("missing repository")
        .issues
        .nodes
        .expect("issue nodes is null")
    {
        if let Some(issue) = issue {
            table.add_row(row!(issue.title, issue.comments.total_count));
        }
    }

    table.printstd();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_repo_name_works() {
        assert_eq!(
            parse_repo_name("graphql-rust/graphql-client").unwrap(),
            ("graphql-rust", "graphql-client")
        );
        assert!(parse_repo_name("abcd").is_err());
    }

    #[test]
    fn create_info_works() -> Result<(), Error> {
        create_info()
    }
}
