FROM debian:10-slim

ENV APP_HOME /app
ENV REPO_DIR /repositories

WORKDIR $APP_HOME
EXPOSE 1965

RUN apt update --fix-missing \
	&& apt install -y libssl1.1 \
	&& rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

ADD . $APP_HOME

RUN mkdir -p /root/.config/config \
	&& mv deploy/gemini-git-browser.toml /root/.config/config \
	&& mv deploy/robots.txt /app/static/ \
	&& printf "RUST_LOG=northstar=debug bin/gemini-git-browser\n" > start.sh

CMD ["sh", "start.sh"]
