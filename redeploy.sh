#!/bin/bash
docker build . -t statsoverlay 
docker compose down --remove-orphans
docker compose up -d
