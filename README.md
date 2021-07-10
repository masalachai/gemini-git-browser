# Gemini Git Browser

The gemini git browser is a very basic UI to browse git repositories on the gemini protocol

It can be seen in action at gemini://git.ritesh.ch

## Demo

A local demo can be run by cloning the repository and build a local docker container that will host a sample repository within the container.

```
git clone git@github.com:masalachai/gemini-git-browser.git && cd gemini-git-browser
sh deploy/deploy.sh development
docker run -p 1965:1965 -it gemini_git_browser
```

Once the container is running, the capsule can be seen with a gemini browser at gemini://localhost/

## Setup

To use it with your repos, please note the following.

1. Currently, it only works with namespaced repos, i.e. the repo directories must be placed under a parent directory that acts as its namespace. For example `masalachai/gemini-git-browser` where `masalachai` is the namespace and `gemini-git-browser` the repo.

2. A `REPO_DIR` environment variable needs to be set to the path of these namespace dirs. For example, if your repository is at `/repositories/masalachai/gemini-git-browser`, `REPO_DIR` should be set as follows

```
REPO_DIR=/repositories
```

3. A `config.toml` file also needs to be created and placed at `$HOME/.config/config`. Within the `config.toml` there must be a list of namespace dirs and repo dirs which are allowed to be served.

```
# $HOME/.config/config/config.toml
allowed = allowed = ["masalachai", "masalachai/gemini-git-browser"]
```

One the `REPO_DIR` variable and `config.toml` file is set, executing the binary should serve the repos at the gemini port.

## License

The source is released under the GNU GPL v3, a copy of which is included in the repository
