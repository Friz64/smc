#!/bin/bash

name=smc
output=packed/
temp=temp/
declare -a folders=("assets/" "config/")

# https://stackoverflow.com/a/33826763
while [[ "$#" > 0 ]]; do case $1 in
    -l|--linux) linux=1;;
    -w|--windows) windows=1;;
    *) echo "Unknown parameter passed: $1"; exit 1;;
esac; shift; done

if [ ! -d "$output" ]; then
    mkdir $output
fi

if [ "$linux" != "1" ] && [ "$windows" != "1" ]; then
    echo "No build options specified"
fi

if [ "$linux" == "1" ]; then
    echo "BUILDING LINUX"
    cargo build --release
    strip target/release/$name

    echo ""
    echo "PACKING LINUX"

    mkdir $temp/
    for folder in "${folders[@]}"; do
        cp -r $folder ${temp}/$folder
    done
    cp target/release/$name $temp

    cd $temp
    zip -r ${name}_linux.zip .
    cd ..

    mv ${temp}${name}_linux.zip $output
    rm -r $temp

    echo ""
fi

if [ "$windows" == "1" ]; then
    echo "BUILDING WINDOWS"
    cargo build -p $name --release --target x86_64-pc-windows-gnu
    strip target/x86_64-pc-windows-gnu/release/$name.exe

    echo ""
    echo "PACKING WINDOWS"

    mkdir $temp/
    for folder in "${folders[@]}"; do
        cp -r $folder ${temp}/$folder
    done
    cp target/x86_64-pc-windows-gnu/release/$name.exe $temp
    
    cd $temp
    zip -r ${name}_windows.zip .
    cd ..

    mv ${temp}/${name}_windows.zip $output
    rm -r $temp

    echo ""
fi
