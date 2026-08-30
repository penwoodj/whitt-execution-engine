#!/bin/sh
count=$(ps -eo stat,args | grep -v 'Z' | grep llama-server | grep -v -- --models-dir | grep -c llama-server)
echo "${count:-0}"
