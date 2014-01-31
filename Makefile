all:
	for item in *.rs; \
	do \
		echo Compiling $$item; \
		rustc $$item -o /tmp/tmp || exit; \
		echo Compiled $$item; \
		echo; \
	done