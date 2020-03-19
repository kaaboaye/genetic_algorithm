#! /usr/bin/env bash
set -e

# set locale to us in order to fix seq formatting
LANG=en_US
LC_NUMERIC=en_US.UTF-8

GENET='../target/release/genet'
SCENARIO='../scenario.txt'

rm `ls res* 2> /dev/null` 2> /dev/null || echo -n

for param in `seq 1000 1000 10000`; do
    for iteration in `seq 1 5`; do
        echo param $param iteration $iteration
        
        $GENET train $SCENARIO \
        --crossover-probability 0.8 \
        --generation-limit 20 \
        --mutation-probability 0.003 \
        --population-size $param \
        --tournament-size $(($param * 20 / 100)) \
        --epsilon 0.000000000000000000000001 \
        res_$param\_$iteration
    done
done

python ./plot.py
