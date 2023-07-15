#!/bin/sh

directory_name=${PWD##*/}
tmp_path="/mnt/c/Windows/temp/$directory_name"

mkdir -p $tmp_path

rsync . $tmp_path -r --exclude-from=.gitignore

cd $tmp_path

powershell.exe -Command "cargo run $@"
