#!/usr/bin/env bash
set -o errexit
set -o pipefail
set -o nounset

export PROJECT=caucus-project
export PRODUCT=caucus-product
export KUBE_CONTEXT=caucus-context
export KUBE_NAMESPACE=caucus-namespace

# TODO: what does "@" do for us
source donkey/deploy.sh "$@" 
