export PROJECT_ALIAS="caucus"

source ${CAUCUS_PATH}/clients/python/venv/bin/activate;

alias cdf="${PROJECT_ALIAS}; cd apps/web";
alias cdb="${PROJECT_ALIAS}; cd services/api";
alias cdp="${PROJECT_ALIAS}; cd clients/python";

alias sf="cdf; npm run dev";
alias sb="cdb; cargo run";
alias fenv="cdb; source flush_env.sh";

