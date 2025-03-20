include config.mk

all: build

build:
	cargo +nightly build --release
ifeq ($(ENABLE_DOCS),1)
	cd docs && mdbook build
endif

clean:
	rm -f config.mk
	cargo clean
ifeq ($(ENABLE_DOCS),1)
	cd docs && mdbook clean
endif

install:
	install -Dm755 target/release/two 	$(DESTDIR)$(LIBEXECDIR)/two
	install -Dm755 launch.sh 		  	$(DESTDIR)$(BINDIR)/2

ifeq ($(ENABLE_CONF),1)
	install -Dm644 config.toml 		 	$(DESTDIR)/$(SYSCONFDIR)/2/config.toml
	install -Dm644 exclusions.txt 	 	$(DESTDIR)/$(SYSCONFDIR)/2/exclusions.txt
	install -Dm644 repo_priority.txt 	$(DESTDIR)/$(SYSCONFDIR)/2/repo_priority.txt
endif

ifeq ($(ENABLE_COMP),1)
	install -Dm644 completions/bash 	$(DESTDIR)/usr/share/bash-completion/completions/2
	install -Dm644 completions/zsh  	$(DESTDIR)/usr/share/zsh/site-functions/_2
	install -Dm644 completions/fish 	$(DESTDIR)/usr/share/fish/vendor_completions.d/2.fish
endif

ifeq ($(ENABLE_DOCS),1)
	rm -rf 		$(DESTDIR)$(DOCDIR)
	cp -a docs 	$(DESTDIR)$(DOCDIR)
endif

ifeq ($(ENABLE_MAIN),1)
    git clone --depth=1 --single-branch --branch master https://github.com/Toxikuu/2-main.git $(DESTDIR)/var/ports/main
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
