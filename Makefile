include config.mk

all: build

build:
	@echo "Building 2..."
	cargo +nightly build --release
ifeq ($(ENABLE_DOCS),1)
	@echo "Building documentation..."
	cd docs && mdbook build
endif

clean:
	rm -f config.mk
	cargo clean
ifeq ($(ENABLE_DOCS),1)
	cd docs && mdbook clean
endif

install:
	@echo "Installing executables..."
	install -Dm755 target/release/two 	$(DESTDIR)$(LIBEXECDIR)/two
	install -Dm755 launch.sh 		  	$(DESTDIR)$(BINDIR)/2

	@echo "Installing environment files..."
	install -Dm644 envs/cmake			$(DESTDIR)/usr/share/2/envs/cmake
	install -Dm644 envs/core			$(DESTDIR)/usr/share/2/envs/core
	install -Dm644 envs/meson			$(DESTDIR)/usr/share/2/envs/meson
	install -Dm644 envs/ninja			$(DESTDIR)/usr/share/2/envs/ninja
	install -Dm644 envs/pip				$(DESTDIR)/usr/share/2/envs/pip
	install -Dm644 envs/rust			$(DESTDIR)/usr/share/2/envs/rust
	install -Dm644 envs/xorg			$(DESTDIR)/usr/share/2/envs/xorg

ifeq ($(ENABLE_CONF),1)
	@echo "Installing configuration files..."
	install -Dm644 etc/config.toml 		 $(DESTDIR)$(SYSCONFDIR)/2/config.toml
	install -Dm644 etc/exclusions.txt 	 $(DESTDIR)$(SYSCONFDIR)/2/exclusions.txt
	install -Dm644 etc/repo_priority.txt $(DESTDIR)$(SYSCONFDIR)/2/repo_priority.txt
endif

ifeq ($(ENABLE_COMP),1)
	@echo "Installing shell completions..."
	install -Dm644 completions/bash 	$(DESTDIR)/usr/share/bash-completion/completions/2
	install -Dm644 completions/zsh  	$(DESTDIR)/usr/share/zsh/site-functions/_2
	install -Dm644 completions/fish 	$(DESTDIR)/usr/share/fish/vendor_completions.d/2.fish
endif

ifeq ($(ENABLE_DOCS),1)
	@echo "Installing documentation..."
	rm -rf 				$(DESTDIR)$(DOCDIR)
	mkdir -p 			$(DESTDIR)$(DOCDIR)
	cp -a docs/book/* 	$(DESTDIR)$(DOCDIR)
endif

ifeq ($(ENABLE_MAIN),1)
	@if [ -d "$(DESTDIR)/var/ports/main" ]; then \
		echo "Main package repo exists, skipping clone."; \
	else \
		echo "Cloning main package repo..."; \
		git clone --depth=1 --single-branch --branch master https://github.com/Toxikuu/2-main.git $(DESTDIR)/var/ports/main; \
	fi
endif

uninstall:
	rm -f $(DESTDIR)$(LIBEXECDIR)/two
	rm -f $(DESTDIR)$(BINDIR)/2

ifeq ($(ENABLE_CONF),1)
	rm -rf $(DESTDIR)$(SYSCONFDIR)/2
endif

ifeq ($(ENABLE_COMP),1)
	rm -f $(DESTDIR)/usr/share/bash-completion/completions/2
	rm -f $(DESTDIR)/usr/share/zsh/site-functions/_2
	rm -f $(DESTDIR)/usr/share/fish/vendor_completions.d/2.fish
endif

ifeq ($(ENABLE_DOCS),1)
	rm -rf $(DESTDIR)$(DOCDIR)
endif

ifeq ($(ENABLE_MAIN),1)
	rm -rf $(DESTDIR)/var/ports/main
endif

	@echo "If you installed additional repos, you may want to manually remove those"
