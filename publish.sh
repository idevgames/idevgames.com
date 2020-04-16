#!/bin/sh

REMOTE_USER=xpm
REMOTE_HOST=mysteriouspants.com
REMOTE_PATH=/var/www/www.idevgames.com

zola build

ssh ${REMOTE_USER}@${REMOTE_HOST} sudo chown xpm:xpm ${REMOTE_PATH}
rsync -avz public/ ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}/
ssh ${REMOTE_USER}@${REMOTE_HOST} "find ${REMOTE_PATH} -type d -exec chmod 755 {} \;"
ssh ${REMOTE_USER}@${REMOTE_HOST} "find ${REMOTE_PATH} -type f -exec chmod 644 {} \;"

