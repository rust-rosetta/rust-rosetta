RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
SRCDIR:=src
DSTDIR:=/tmp/rosetta
SRC:=$(wildcard $(SRCDIR)/*.rs)
PROGDIR:=dist
PROG:=$(patsubst $(SRCDIR)/%.rs,$(PROGDIR)/%,$(SRC))

.SILENT:

all:
	# Make a directory
	mkdir -p /tmp/rosetta/src;
	# Test compiling executables
	for item in $(SRCDIR)/*.rs; \
	do \
		echo Compiling $$item; \
		rustc --test $$item -o $(DSTDIR)/$$item || exit; \
		echo Compiled $$item; \
		echo; \
	done;
	for item in $(DSTDIR)/src/*.rs; \
	do \
		echo Testing $$item; \
		$$item; \
		echo Tested $$item; \
		echo; \
	done;

changed:
	# Make files which changed from master
	for item in $$(git diff --name-only master..HEAD | grep ".rs$$"); \
	do \
		echo Compiling $$item; \
		rustc --test $$item -o /tmp/rosetta/$$item || exit; \
		echo Compiled $$item; \
		echo; \
		echo Testing $$item; \
		$(DSTDIR)/$$item; \
		echo Tested $$item; \
	done;

help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

clean:
	# Remove executables
	rm -fr $(PROG)

exe: $(PROG)
	# Build executables

$(PROGDIR):
	mkdir $(PROGDIR)

$(PROGDIR)/% : $(SRCDIR)/%.rs | $(PROGDIR)
	$(RUSTC) $(RUSTFLAGS) $< -o $@
