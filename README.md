# Gemini Git Browser

The gemini git browser is a very basic UI to browse git repositories on the gemini protocol

It can be seen in action at gemini://git.ritesh.ch

Or on https://proxy.vulpes.one/gemini/git.ritesh.ch/ if you're on an HTTP browser.

If the tree contains a README.md file, like github and other popular web UIs for git, it will automatically convert and render the README.md to the gemini page.

![Single Repo View](https://ci.ayravat.com/filestore/single-repo.jpeg) ![File View](https://ci.ayravat.com/filestore/file.jpeg) ![Repo List View](https://ci.ayravat.com/filestore/repos.jpeg) ![Readme View](https://ci.ayravat.com/filestore/readme.jpeg)

## CI Builds

[CI for the project](https://ci.ayravat.com/ritesh/gemini-git-browser)

## Demo

A local demo can be run by cloning the repository and building a local docker container that will host a sample repository within the container.

```
git clone git@github.com:masalachai/gemini-git-browser.git && cd gemini-git-browser
sh deploy/deploy.sh development
docker run -p 1965:1965 -it gemini_git_browser:latest
```

Once the container is running, the capsule can be seen with a gemini browser at gemini://localhost/

## Setup

To use it with your repos, please note the following.

1. Currently, it only works with namespaced repos, i.e. the repo directories must be placed under a parent directory that acts as its namespace. For example `masalachai/gemini-git-browser` where `masalachai` is the namespace and `gemini-git-browser` the repo.

2. A `REPO_DIR` environment variable needs to be set to the path of these namespace dirs. For example, if your repository is at `/repositories/masalachai/gemini-git-browser`, `REPO_DIR` should be set as follows

```
REPO_DIR=/repositories
```

3. A `config.toml` file also needs to be created and placed at `$HOME/.config/config`. Within the `config.toml` there must be a list of namespace dirs and repo dirs which are allowed to be served, and a title, which may be left blank if unneeded.

```
# $HOME/.config/config/config.toml
allowed = ["masalachai", "masalachai/gemini-git-browser"]
title = ""
```

Once the `REPO_DIR` variable and `config.toml` file is set, executing the binary should serve the repos at the gemini port.

## Run from Docker

The UI is also packaged as a docker image for easy deployment. To run it, create the `config.toml` file and a TLS certificate and private key first (gemini uses TLS by default), then mount the `config.toml`, the certificate and private key and your repository directory into the container with the following command.

```
# Paths used for the certificate and key are default let's encrypt paths
docker run -p 1965:1965 \
	-v /etc/letsencrypt/live/yourdomain.com/fullchain.pem:/app/cert/cert.pem \
	-v /etc/letsencrypt/live/yourdomain.com/privkey.pem:/app/cert/key.pem \
	-v /my-repositories:/repositories
	-v $HOME/.config/config/config.toml:/root/.config/config/config.toml
	-it ayravat/gemini-git-browser:latest
```

Once the container is running you can access the UI at gemini://localhost/

## License

The source is released under the GNU GPL v3, a copy of which is included in the repository
