build-vault:
	rm -rf vault
	rm -rf plugin/dist
	mkdir -p vault
	mkdir -p vault/.obsidian/plugins/yeff
	ln -s $$PWD/vault/.obsidian/plugins/yeff ./plugin/dist
	wget https://github.com/pjeby/hot-reload/releases/download/0.2.1/hot-reload.zip -O vault/hot-reload.zip
	cd vault/.obsidian/plugins && unzip ../../hot-reload.zip
