#! /usr/bin/env bash
set -e

# set locale to us in order to fix seq formatting
LANG=en_US
LC_NUMERIC=en_US.UTF-8

GENET='../target/release/genet'
SCENARIO='../scenario.txt'

rm `ls res* 2> /dev/null` 2> /dev/null || echo -n

for population_size in `echo 10 25 50 75 100`; do
    for iteration in `seq 1 10`; do
        # population_size=500
        
        $GENET train $SCENARIO \
        --crossover-probability 0.85 \
        --generation-limit 100 \
        --mutation-probability 0.003 \
        --population-size $population_size \
        --tournament-size $(($population_size * 20 / 100)) \
        res_$population_size\_$iteration
    done
done

python ./plot.py
