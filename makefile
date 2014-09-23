#globals
default: build
freshen: clean build
clean:
	rm -rf bin/*

#vars
options = -A dead_code -A unused_variable

#includes
include lists.mk

#compilation definitions
$(binaries): bin/% : src/%.rs
	rustc $(options) -g $< -o $@

#commands
build: $(binaries)

#tests
test: bin/test
	$<

test-asdf: bin/asdf
	$<

test-go-ai: bin/go_ai
	$<
