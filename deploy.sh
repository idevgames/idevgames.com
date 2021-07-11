#!/bin/bash

set -o xtrace
set -e

SERVICE_SU_USER=xpm
SERVICE_HOST=mysteriouspants.com
SERVICE_USER_NAME=idevgames
SERVICE_USER_DIR=/home/idevgames
SERVICE_NAME=idevgames

# backup the prod database
scp ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/db/app.sqlite $(pwd)/db/app-$(date +%s).sqlite

ssh ${SERVICE_SU_USER}@${SERVICE_HOST} sudo apt-get install libssl1.1 libsqlite3-0

# the main reason for containerizing the build is so that we can
# target the os the idevgames server runs rather than whatever your
# local machine happens to be running
docker run --rm --name idevgamesc \
  -u $(id -u ${USER}):$(id -g ${USER}) \
  -v $(pwd):/src \
  -w /src \
  rust:1.53 \
  cargo build --release

pushd react-project
  npm run-script build
popd

# deploy the build if successful
ssh ${SERVICE_SU_USER}@${SERVICE_HOST} sudo systemctl stop ${SERVICE_NAME}
ssh ${SERVICE_USER_NAME}@${SERVICE_HOST} mkdir -p ${SERVICE_USER_DIR}/db
rsync -avzr target/release/idevgames ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/idevgames
rsync -avzr --delete react-project/build/ ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/static/
ssh ${SERVICE_USER_NAME}@${SERVICE_HOST} chmod +x ${SERVICE_USER_DIR}/idevgames
ssh ${SERVICE_USER_NAME}@${SERVICE_HOST} ${SERVICE_USER_DIR}/idevgames migrate
ssh ${SERVICE_SU_USER}@${SERVICE_HOST} sudo systemctl start ${SERVICE_NAME}
