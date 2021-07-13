#[macro_use]
extern crate lazy_static;

pub mod util;
pub mod repo;

pub mod actions {
    use northstar::{GEMINI_MIME, Body, Request, Response};
    use anyhow::anyhow;
    use futures_core::future::BoxFuture;
    use futures_util::FutureExt;
    use handlebars::Handlebars;
    use serde_json::json;
    use mime;

    use crate::repo;

    async fn get_html(name: &str, template_path: &str, json: &serde_json::Value) -> anyhow::Result<Response> {
        let mut hb = Handlebars::new();

        hb.register_template_file(name, template_path).unwrap_or_else(|err| {
            anyhow!("Template registration error: {}", err);
        });

        match hb.render(name, json) {
            Ok(contents) => Ok(Response::success(&GEMINI_MIME, Body::from(contents))),
            Err(err) => Err(anyhow!("Template rendering error: {}", err))
        }
    }

    async fn get_blob(path_segments: &Vec<String>) -> anyhow::Result<Response> {
        let repo_path = format!("{}/{}", path_segments[0], path_segments[1]);
        let repo = repo::Repo::new(&repo_path).unwrap();

        let blob = repo.get_blob(&path_segments[3]).unwrap();

        let application_octet_stream = "application/octet-stream".parse::<mime::Mime>().unwrap();
        let text_gemini = "text/plain".parse::<mime::Mime>().unwrap();

        let mime: northstar::types::Mime = match blob.is_binary {
            true => application_octet_stream,
            false => text_gemini
        };

        Ok(Response::success(&mime, blob.content))
    }

    async fn get_tree(path_segments: &Vec<String>) -> anyhow::Result<Response> {
        let repo_path = format!("{}/{}", path_segments[0], path_segments[1]);
        let repo = repo::Repo::new(&repo_path).unwrap();

        let tree_response = repo.get_tree(&path_segments[3]).unwrap();

        get_html(
            "branch",
            "./templates/tree.hbs",
            &json!({
                "path": repo_path,
                "tree": tree_response.tree,
                "readme": tree_response.readme_text
            })
        ).await
    }

    async fn get_branch(path_segments: &Vec<String>) -> anyhow::Result<Response> {
        let repo_path = format!("{}/{}", path_segments[0], path_segments[1]);
        let repo = repo::Repo::new(&repo_path).unwrap();

        let branch = &path_segments[3];

        let tree_response = repo.get_branch_tree(&branch).unwrap();

        get_html(
            "branch",
            "./templates/tree.hbs",
            &json!({
                "path": repo_path,
                "title": branch,
                "tree": tree_response.tree,
                "readme": tree_response.readme_text
            })
        ).await
    }

    async fn get_repo(path: &str) -> anyhow::Result<Response> {
        let repo = repo::Repo::new(path).unwrap();

        let repo_details = repo.get_details().unwrap();

        get_html(
            "repo",
            "./templates/repo.hbs",
            &json!({
                "path": path,
                "details": repo_details
            })
        ).await
    }

    async fn get_repo_list(path: &str) -> anyhow::Result<Response> {
        let repos = repo::REPO_DIR.get_ns_repos(path).unwrap();

        let repo_list = serde_json::to_value(&repos).unwrap();
        get_html("ns", "./templates/ns.hbs", &json!({"title": repo::REPO_DIR.config.title, "ns": path, "repos": &repo_list})).await
    }

    async fn get_page(request: Request) -> anyhow::Result<Response> {
        let segments = request
            .path_segments()
            .iter()
            .filter_map(|s| {
                if let 0 = s.len() { None } else { Some(String::from(s)) }
            }
            ).collect::<Vec<String>>();

        let path = segments.join("/");

        match segments.len() {
            1 => get_repo_list(&path).await,
            2 => get_repo(&path).await,
            _ => {
                match segments[2].as_str() {
                    "branch" => get_branch(&segments).await,
                    "tree" => get_tree(&segments).await,
                    "blob" => get_blob(&segments).await,
                    _ => Ok(Response::not_found())
                }
            }
        }
    }

    pub fn index<'a>(request: Request) -> BoxFuture<'a, anyhow::Result<Response>> {
        async move {
            if let 0 = request.trailing_segments().len() {
                let repos = match repo::REPO_DIR.get_repos() {
                    Ok(repos) => repos,
                    Err(e) => return Err(anyhow!("Error fetching repo list: {}", e))
                };

                let repos_json = serde_json::to_value(&repos).unwrap();

                get_html("index", "./templates/index.hbs", &json!({"title": repo::REPO_DIR.config.title, "repos": repos_json})).await
            } else {
                get_page(request).await
            }
        }
        .boxed()
    }

    pub fn favicon<'a>(_: Request) -> BoxFuture<'a, anyhow::Result<Response>> {
        async move {
            Ok(Response::success(&GEMINI_MIME, Body::from("\u{1F4DA}\r\n")))
        }
        .boxed()
    }

    pub fn robots<'a>(_: Request) -> BoxFuture<'a, anyhow::Result<Response>> {
        async move {
            let response = northstar::util::serve_file("./static/robots.txt", &mime::TEXT_PLAIN).await?;

            Ok(response)
        }
        .boxed()
    }
}
