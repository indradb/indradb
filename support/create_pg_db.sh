#!/bin/bash
set -e

if [ -z "$DATABASE_OWNER" ]; then
    createdb $DATABASE_NAME
    psql -d $DATABASE_NAME -a -f support/schema.sql
else
    createdb --owner=$DATABASE_OWNER $DATABASE_NAME
    psql -U $DATABASE_OWNER -d $DATABASE_NAME -a -f support/schema.sql
fi
