#!/bin/bash
projects="01 02 03 step04 step05 step06 step07 step08 step09 step10"
home=$(pwd)
for project in $projects; do
    cd $project
    cargo fmt
    cargo build
    cd $home
done
