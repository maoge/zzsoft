#! /bin/sh

#RUST_LOG=debug 


#export RUST_BACKTRACE=0
export RUST_BACKTRACE=1
#export RUST_BACKTRACE=full

nohup ./r_zz_metasvr >> ./log/zz_metasvr.log 2>&1 &
