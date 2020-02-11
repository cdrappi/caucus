#!/usr/bin/env sh

app_name="open-caucus-staging"

docker build -t open-caucus .
heroku container:push web --app ${app_name}
heroku container:release web --app ${app_name}

