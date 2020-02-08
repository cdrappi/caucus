export PROJECT_ALIAS="caucus"

source ${CAUCUS_PATH}/clients/python/venv/bin/activate;

alias cdf="${PROJECT_ALIAS}; cd apps/web";
alias cdb="${PROJECT_ALIAS}; cd services/api";
alias cdp="${PROJECT_ALIAS}; cd clients/python";

alias sf="cdf; npm run dev";
alias bb="cdb; cargo build";
alias bbr="bb --release";
alias sb="cdb; cargo run";
alias sbr="sb --release";
alias fenv="cdb; source flush_env.sh";

