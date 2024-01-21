#!/bin/bash
# start docker in host network state

docker run --network host --name "dst-server"  -d <image>