# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {
    cargo build
    cargo build --release

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    cargo test
    cargo test --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG] && [ "$TARGET" != "x86_64-apple-darwin" ]; then
    main
fi
