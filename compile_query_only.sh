#!/bin/bash
#This is just an example compilation.  You should integrate these files into your build system.  Boost jam is provided and preferred.

set -e

CXX=${CXX:-g++}

CXXFLAGS+=" -I. -O3 -DNDEBUG -DKENLM_MAX_ORDER=6"

#If this fails for you, consider using bjam.
echo 'Compiling with '$CXX $CXXFLAGS

#Grab all cc files in these directories except those ending in test.cc or main.cc
objects=""
for i in  ./c_api/load.cpp $ADDED_PATHS; do
    $CXX $CXXFLAGS -c $i -fPIC -o ${i%.cpp}.o
    objects="$objects ${i%.cpp}.o"
done
LDFLAGS="-L./build/lib/"
$CXX $objects -o lib/libkenlmrust.so $CXXFLAGS $LDFLAGS -lkenlm -shared
