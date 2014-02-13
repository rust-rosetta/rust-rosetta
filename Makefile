RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
SRC=$(wildcard *.rs)
PROG:=$(patsubst %.rs,%,$(SRC))

.SILENT:

all:
	# Test compiling executables
	for item in *.rs; \
	do \
		echo Compiling $$item; \
		rustc $$item -o /tmp/tmp || exit; \
		echo Compiled $$item; \
		echo; \
	done

help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

clean:
	# Remove executables
	rm -fr $(PROG)

exe: $(PROG)
	# Build executables

% : %.rs
	$(RUSTC) $(RUSTFLAGS) $< -o $@
