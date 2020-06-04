#!/bin/sh

echo $1 | vegeta attack -rate 6 -workers 13 -duration 10m