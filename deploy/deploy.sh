#!/bin/sh

VERSION=0.6.0
DEPLOY_SCRIPTS_GIT_REPO=https://github.com/loanstreet/deploy-scripts.git
DEPLOY_SCRIPTS_GIT_BRANCH="$VERSION"
DEPLOY_SCRIPTS_HOME="$HOME/.deploy-scripts/$DEPLOY_SCRIPTS_GIT_BRANCH"
SCRIPT_PATH=$(dirname $(readlink -f $0))

if [ ! -d $DEPLOY_SCRIPTS_HOME ]; then
	mkdir -p $DEPLOY_SCRIPTS_HOME
	echo "Downloading deploy-scripts"
	GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no" git clone --single-branch --branch $VERSION $DEPLOY_SCRIPTS_GIT_REPO $DEPLOY_SCRIPTS_HOME
fi
cd $DEPLOY_SCRIPTS_HOME && GIT_SSH_COMMAND="ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no" git fetch origin +refs/heads/$DEPLOY_SCRIPTS_GIT_BRANCH && git checkout $DEPLOY_SCRIPTS_GIT_BRANCH && cd $SCRIPT_PATH
PROJECT_DEPLOY_DIR=$SCRIPT_PATH sh $DEPLOY_SCRIPTS_HOME/deploy.sh $1
