export PROJECT_ALIAS="caucus"
export PYTHONPATH="${CAUCUS_PATH}/clients/python";

source ${CAUCUS_PATH}/clients/python/venv/bin/activate;

alias cdf="${PROJECT_ALIAS}; cd apps/web";
alias cdb="${PROJECT_ALIAS}; cd services/api";
alias cdp="${PROJECT_ALIAS}; cd clients/python";

alias sf="cdf; npm run dev";
alias bb="cdb; cargo build";
alias bbr="bb --release";
alias sb="cdb; cargo run";
alias sbr="sb --release";

alias flush_env_backend="cdb; source flush_env.sh";

alias fed="flush_env_backend dev";
alias fes="flush_env_backend staging";

alias dbh="bbr; bash deploy.sh";

alias rdb="cdp; python db/drop_tables.py; cdb; diesel migration run; cdp; python db/seed_db.py; caucus";
