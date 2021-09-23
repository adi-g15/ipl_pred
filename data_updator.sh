#!/bin/bash

# This file is then added to crontab to automatically run regularly at 23:20, and 19:20 daily

cd "$(dirname "$0")" && yarn run sync_data && git add . && git commit -m "Automatic Data Update: $(date)" && git push origin main

