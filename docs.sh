#!/bin/bash

# Build project
cargo build

# Generate documentation
cargo rustdoc -- --no-defaults --passes collapse-docs --passes unindent-comments

# Make temporary directory for gh-pages
rm -rf .gh-pages
mkdir .gh-pages
cd .gh-pages
git init

# Copy docs
cp -r ../target/doc/*
cat <<EOF > index.html
<!DOCTYPE html>
<html>
  <title>Rust-Monopoly</title>
  <meta http-equiv=refresh content=0;url=rust_monopoly/index.html>
</html>
EOF

# Add, commit, and push files
git add -f --all
git commit -m "Built documentation"
git checkout -b gh-pages
git remote add origin git@github.com:jharkins95/Rust-Monopoly.git

# Cleanup
cd .. 
rm -rf .gh-pages 
