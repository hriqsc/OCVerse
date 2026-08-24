#!/usr/bin/env fish

rm -rf test_output.txt
rm -rf db
rm -rf images

cargo nextest run 2>&1 | tee test_output.txt
