all:
	for item in *.rs; \
	do \
		echo Compiling $$item; \
		rustc $$item; \
		echo Compiled $$item; \
		echo; \
	done