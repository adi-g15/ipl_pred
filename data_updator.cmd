@echo on

yarn run sync_data && git add . && git commit -m "Data Update %date%" && git push origin main
