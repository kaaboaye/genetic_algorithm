#! /usr/bin/env bash
set -e

# set locale to us in order to fix seq formatting
LANG=en_US
LC_NUMERIC=en_US.UTF-8

GENET='../target/release/genet'
SCENARIO='../scenario.txt'

for param in `seq 0.25 0.25 0.75`; do
    for iteration in `seq 1 10`; do
        $GENET train $SCENARIO \
        --crossover-probability $param \
        --generation-limit 20 \
        --mutation-probability 0.005 \
        --population-size 100 \
        --tournament-size 25 \
        --epsilon 0.000000000000000000000001 \
        res_$param\_$iteration
    done
done

