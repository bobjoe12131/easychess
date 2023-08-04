# Convenient script to update docs and README.md
cargo doc --no-deps

# from crates.io
if command -v cargo-readme &> /dev/null
then
    cargo-readme readme > ./README.md
fi
