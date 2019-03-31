#!/bin/bash

name=smc
dir=packed/
temp=temp/

if [ ! -d "$dir" ]; then
    mkdir $dir
fi

# https://stackoverflow.com/a/33826763
while [[ "$#" > 0 ]]; do case $1 in
    -l|--linux) linux=1;;
    -w|--windows) windows=1;;
    *) echo "Unknown parameter passed: $1"; exit 1;;
esac; shift; done

if [ "$linux" != "1" ] && [ "$windows" != "1" ]; then
    echo "No build options specified"
fi

if [ "$linux" == "1" ]; then
    echo "BUILDING LINUX"
    cargo build --release
    strip target/release/$name

    echo ""
    echo "PACKING LINUX"
    cp -r assets/ $temp
    cp -r resources/ $temp
    cp target/release/$name $temp
    cd $temp
    zip -r ${name}_linux.zip .
    cd ..
    mv ${temp}/${name}_linux.zip $dir
    rm -r $temp

    echo ""
fi

if [ "$windows" == "1" ]; then
    echo "BUILDING WINDOWS"
    cargo build -p $name --release --target x86_64-pc-windows-gnu
    strip target/x86_64-pc-windows-gnu/release/$name.exe

    echo ""
    echo "PACKING WINDOWS"
    cp -r assets/ $temp
    cp -r resources/ $temp
    cp target/x86_64-pc-windows-gnu/release/$name.exe $temp
    cd $temp
    zip -r ${name}_windows.zip .
    cd ..
    mv ${temp}/${name}_windows.zip $dir
    rm -r $temp

    echo ""
fi
