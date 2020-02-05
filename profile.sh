export PROJECT_ALIAS="rnr"

alias cdf="${PROJECT_ALIAS}; cd apps/web";
alias cdb="${PROJECT_ALIAS}; cd services/api";

alias sf="cdf; npm run dev";
alias sb="cdb; cargo run";
alias fenv="cdb; source flush_env.sh";

