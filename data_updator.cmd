@echo on

yarn run sync_data && git add . && git commit -m "Automatic Data Update: %date%, %time%" && git push origin main
