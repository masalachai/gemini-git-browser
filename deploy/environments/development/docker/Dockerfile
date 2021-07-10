FROM debian:10-slim

ENV APP_HOME /app
ENV REPO_DIR /repositories

WORKDIR $APP_HOME
EXPOSE 1965

RUN apt update --fix-missing \
	&& apt install -y git \
	&& mkdir -p /repositories/repo-ns/sample.git \
	&& cd /repositories/repo-ns/sample.git \
	&& git init --bare \
	&& mkdir -p /tmp/sample \
	&& cd /tmp/sample \
	&& git init \
	&& git config --global user.name "Sample" \
	&& git config --global user.email "dev@sample.test" \
	&& echo "<h1>Hello world</h1>" > index.html \
	&& git add . \
	&& git commit . -m "sample repo" \
	&& git remote add origin file:///repositories/repo-ns/sample.git \
	&& git push -u origin master \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ADD . $APP_HOME

RUN mkdir -p /root/.config/config \
	&& mv deploy/config.toml /root/.config/config \
	&& printf "RUST_LOG=northstar=debug bin/gemini-git-browser\n" > start.sh \
	&& mkdir -p /app/cert \
	&& cd /app/cert \
	&& openssl req -x509 -nodes -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -subj "/C=MY/ST=KL/L=KL/O=/OU=/CN=localhost/emailAddress=admin@localhost"

CMD ["sh", "start.sh"]
