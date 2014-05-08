RUSTC:=rustc
SRC_DIR:=src
BUILD_DIR:=build
TEST_DIR:=build-tests
SOURCES:=$(wildcard $(SRC_DIR)/*.rs)
PROG:=$(patsubst $(SRCDIR)/%.rs,$(PROGDIR)/%,$(SRC))

# Used internally to toggle between building and test-building
build_script_flags:=

# Export variables like SRC_DIR to our shell callouts.  Necessary for
# build.sh.
.EXPORT_ALL_VARIABLES:
.SILENT:

.PHONY: all
all: remove_error_indicator create_build_dir create_test_dir ${SOURCES}
	# Build and test all rust files.  To parallize the build, use make -j8 -O
	echo -e "\nCompilation and testing complete."
	# If these files exist, errors happened so make it our exit
	# code.  We don't use exit codes from individual rust files because
	# that would halt overall compilations.  Since all files are
	# independent, we should continue even if one file fails.
	[ ! -f $(BUILD_DIR)/ERRORS_HAPPENED ] \
	&& [ ! -f $(TEST_DIR)/ERRORS_HAPPENED ]

.PHONY: remove_error_indicator
remove_error_indicator:
	rm -f $(BUILD_DIR)/ERRORS_HAPPENED 2> /dev/null || true; \
	rm -f $(TEST_DIR)/ERRORS_HAPPENED 2> /dev/null || true

# Build rust files individually to allow for parallelization
.PHONY: ${SOURCES}
${SOURCES}:
	./build.sh $(build_script_flags) $@

.PHONY: create_build_dir
create_build_dir:
	mkdir -p $(BUILD_DIR)

.PHONY: create_test_dir
create_test_dir:
	mkdir -p $(TEST_DIR)

.PHONY: build
build: create_build_dir remove_error_indicator toggle-build-flag ${SOURCES}
	# Build all rust files for building and run the builds
	echo -e "\nBuilding complete."
	[ ! -f $(BUILD_DIR)/ERRORS_HAPPENED ]

.PHONY: toggle-build-flag
toggle-build-flag:
	$(eval build_script_flags := -b)

.PHONY: test
test: create_test_dir remove_error_indicator toggle-test-flag ${SOURCES}
	# Build all rust files for testing and run the tests
	echo -e "\nTesting complete."
	[ ! -f $(TEST_DIR)/ERRORS_HAPPENED ]

.PHONY: toggle-test-flag
toggle-test-flag:
	$(eval build_script_flags := -t)

.PHONY: plain
plain: create_build_dir create_test_dir
	# Test compiling executables
	for source in $(SRC_DIR)/*.rs; \
	do \
		item=$$(basename $$source); \
		echo Compiling $$item; \
		rustc $(SRC_DIR)/$$item -o $(BUILD_DIR)/$$item || exit; \
		rustc --test $(SRC_DIR)/$$item -o $(TEST_DIR)/$$item || exit; \
		echo Compiled $$item; \
		echo; \
	done;
	for item in $(TEST_DIR)/*.rs; \
	do \
		echo Testing $$item; \
		$$item; \
		echo Tested $$item; \
		echo; \
	done;

.PHONY: changed
changed: create_build_dir create_test_dir remove_error_indicator \
    $(shell git diff --name-only master | grep ".rs$$")
	# Make files which changed from master

	echo -e "\nCompilation and testing complete of changed files complete."
	[ ! -f $(BUILD_DIR)/ERRORS_HAPPENED ] \
	&& [ ! -f $(TEST_DIR)/ERRORS_HAPPENED ]

.PHONY: help
help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

.PHONY: clean
clean:
	# Remove executables
	rm -fr $(BUILD_DIR)/*
	rm -fr $(TEST_DIR)/*

.PHONY: distclean
distclean: clean
	# Remove executables and delete $BUILD_DIR and $TEST_DIR
	rm -rf $(BUILD_DIR)
	rm -rf $(TEST_DIR)
