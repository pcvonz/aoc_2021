#!/usr/bin/env bash

for i in {5..25}
do
  mkdir src/day_$(printf "%02g" $i)
  cat scaffold > "src/day_$(printf "%02g" $i)/mod.rs"
done

