#!/bin/sh

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

docker run --rm --name idevgamesc \
  -v $(pwd):/src \
  -w /src \
  rust:1.49 \
  cargo build --release

# todo: it would be nice to do brunch in docker so deployment doesn't depend on
# having a working dev setup

brunch build --production

# compile sass/js into production assests
# docker run --rm --name brunchc \
#   -v $(pwd):/home/node/src \
#   -w /home/node/src \
#   node:latest \
#   npm i

# docker run --rm --name brunchc \
#   -v $(pwd):/home/node/src \
#   -w /home/node/src \
#   node:latest \
#   brunch build --production

# deploy the build if successful
ssh ${SERVICE_SU_USER}@${SERVICE_HOST} sudo systemctl stop ${SERVICE_NAME}
ssh ${SERVICE_USER_NAME}@${SERVICE_HOST} mkdir -p ${SERVICE_USER_DIR}/db
rsync -avzr target/release/idevgames ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/idevgames
rsync -avzr --delete static/ ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/static
rsync -avzr --delete templates/ ${SERVICE_USER_NAME}@${SERVICE_HOST}:${SERVICE_USER_DIR}/templates
ssh ${SERVICE_USER_NAME}@${SERVICE_HOST} chmod +x ${SERVICE_USER_DIR}/idevgames
ssh ${SERVICE_SU_USER}@${SERVICE_HOST} sudo systemctl start ${SERVICE_NAME}
