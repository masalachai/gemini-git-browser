use git2::{ObjectType, Oid, Reference, Repository, TreeEntry};
use serde::Serialize;
use std::{collections::HashMap, env, error::Error};

use crate::util;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Tree,
    Blob,
}

#[derive(Serialize, Debug)]
pub struct TreeItem {
    id: String,
    name: String,
    item_type: ItemType,
    icon: String,
}

pub struct Blob {
    pub is_binary: bool,
    pub content: Vec<u8>,
}

#[derive(Serialize, Debug)]
pub struct RepoDetails {
    active_branch: String,
    branches: Vec<String>,
    tree: Vec<TreeItem>,
}

pub struct RepoDir {
    dir_path: String,
    config: util::Config,
}

pub struct Repo<'a> {
    _repo_dir: &'a RepoDir,
    dir_path: String,
    repo: Repository,
    pub name: String,
}

lazy_static! {
    pub static ref REPO_DIR: RepoDir = RepoDir::new();
}

impl<'a> Repo<'a> {
    pub fn new(repo_path: &str) -> Result<Repo, Box<dyn Error>> {
        let dir_path = REPO_DIR.get_repo_dir(repo_path)?;
        let repo = Repository::open(&dir_path)?;
        let segments: Vec<&str> = repo_path.split("/").collect();
        let name = segments[1].trim_end_matches(".git");

        Ok(Repo {
            dir_path: dir_path,
            _repo_dir: &REPO_DIR,
            repo: repo,
            name: String::from(name),
        })
    }

    pub fn get_blob(&self, hash: &str) -> Result<Blob, Box<dyn Error>> {
        let oid = Oid::from_str(hash)?;
        let object = self.repo.find_object(oid, Some(ObjectType::Blob))?;

        let git_blob = object.peel_to_blob()?;
        let contents = git_blob.content().to_owned();

        Ok(Blob {
            is_binary: git_blob.is_binary(),
            content: contents,
        })
    }

    fn filter_tree(item: TreeEntry) -> Option<TreeItem> {
        let name = String::from(item.name().unwrap());
        let oid_str = item.id().to_string();
        match item.kind() {
            Some(ObjectType::Tree) => Some(TreeItem {
                id: oid_str,
                name: name,
                item_type: ItemType::Tree,
                icon: String::from("\u{1F4C1}"),
            }),
            Some(ObjectType::Blob) => Some(TreeItem {
                id: oid_str,
                name: name,
                item_type: ItemType::Blob,
                icon: String::from("\u{1F4C4}"),
            }),
            _ => None,
        }
    }

    pub fn get_tree(&self, hash: &str) -> Result<Vec<TreeItem>, Box<dyn Error>> {
        let oid = Oid::from_str(hash)?;
        let object = self.repo.find_object(oid, Some(ObjectType::Tree))?;

        let tree = object
            .peel_to_tree()?
            .iter()
            .filter_map(Repo::filter_tree)
            .collect::<Vec<TreeItem>>();

        Ok(tree)
    }

    fn get_ref_tree(&self, reference: &Reference) -> Result<Vec<TreeItem>, Box<dyn Error>> {
        let tree = reference
            .peel_to_tree()?
            .iter()
            .filter_map(Repo::filter_tree)
            .collect::<Vec<TreeItem>>();

        Ok(tree)
    }

    pub fn get_details(&self) -> Result<RepoDetails, Box<dyn Error>> {
        let head = self.repo.head()?;
        let active_branch = head.shorthand().unwrap();

        let refs_path = format!("{}/refs/heads", self.dir_path);
        let branches = util::get_files(&refs_path)?;

        let tree = self.get_ref_tree(&head)?;

        Ok(RepoDetails {
            active_branch: String::from(active_branch),
            branches: branches,
            tree: tree,
        })
    }

    pub fn get_branch_tree(&self, branch: &str) -> Result<Vec<TreeItem>, Box<dyn Error>> {
        let ref_path = format!("refs/heads/{}", branch);

        let branch_ref = self.repo.find_reference(&ref_path)?;

        let tree = self.get_ref_tree(&branch_ref)?;

        Ok(tree)
    }
}

impl RepoDir {
    pub fn new() -> RepoDir {
        let repo_dir = env::var("REPO_DIR").unwrap_or_else(|err| {
            panic!("Error reading REPO_DIR: {}", err);
        });

        let config: util::Config = match confy::load("config") {
            Ok(cfg) => cfg,
            Err(e) => panic!("Config read error: {}", e),
        };

        RepoDir {
            dir_path: repo_dir,
            config: config,
        }
    }

    fn get_repo_dir(&self, repo_path: &str) -> Result<String, Box<dyn Error>> {
        if !self.config.check_dir(repo_path) {
            Err(format!("No such repo: {}", repo_path).into())
        } else {
            Ok(format!("{}/{}", self.dir_path, repo_path))
        }
    }

    pub fn get_ns_repos(&self, ns: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let dir = self.get_repo_dir(ns)?;
        util::get_dirs(&dir)
    }

    pub fn get_repos(&self) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
        let ns_dirs = util::get_dirs(&self.dir_path)?;

        let mut repos: HashMap<String, Vec<String>> = HashMap::new();

        for ns_dir in ns_dirs {
            let ns_repo_dir = match self.get_repo_dir(ns_dir.as_str()) {
                Ok(dir) => dir,
                Err(_) => continue,
            };
            match util::get_dirs(&ns_repo_dir) {
                Ok(result) => {
                    let result = result
                        .iter()
                        .filter_map(|repo| {
                            let repo_path = format!("{}/{}", &ns_dir, repo);
                            match self.config.check_dir(repo_path.as_str()) {
                                true => Some(String::from(repo)),
                                false => None,
                            }
                        })
                        .collect::<Vec<String>>();
                    &repos.insert(String::from(ns_dir), result);
                    ()
                }
                Err(e) => return Err(format!("Error reading REPO_DIR: {}", e).into()),
            }
        }

        Ok(repos)
    }
}
