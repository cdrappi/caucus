export PROJECT_ALIAS="rnr"

alias cdm="${PROJECT_ALIAS}; cd apps/mobile";
alias cdb="${PROJECT_ALIAS}; cd services/api";

alias sm="${PROJECT_ALIAS}; cd apps/mobile; expo start --ios";
alias sb="${PROJECT_ALIAS}; cd services/api; cargo run";
alias fenv="cdb; source flush_env.sh";

