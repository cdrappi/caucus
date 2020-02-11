#!/bin/sh

cd ../../;

# Flush our yaml.env to .env in this directory so rust's dotenv can detect configs
python donkey/yaml_to_env.py --yaml=env.yaml --path=services/api/.env --service=api --env=$1;

# Set the special ROCKET_DATABASES env variable from $DATABASE_URL
cd services/api;

source .env;

echo '' >> .env
echo "export ROCKET_DATABASES={postgres_database={url=${DATABASE_URL}}}" >> .env;

source .env;

