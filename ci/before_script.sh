#!/bin/bash

if [ $TRAVIS_OS_NAME = osx ]; then
    rm -rf /usr/local/var/postgres
    initdb /usr/local/var/postgres
    pg_ctl -D /usr/local/var/postgres -w start
    createuser -s postgres
fi

psql -c 'create database braid_test;' -U postgres
