#!/usr/bin/env bash
set -o errexit
set -o pipefail
set -o nounset

export PROJECT=rnr-project
export PRODUCT=rnr-product
export KUBE_CONTEXT=test-context
export KUBE_NAMESPACE=test-namespace

# TODO: what does "@" do for us
source donkey/deploy.sh "$@" 
