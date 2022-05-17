#!/usr/bin/env xonsh

from os.path import dirname,abspath
import platform
PWD = dirname(abspath(__file__))
cd @(PWD)

p".xonshrc".exists() && source .xonshrc

system = platform.system().lower()
if system == 'darwin':
  system = f'apple-{system}'
elif system == 'linux':
  system = 'unknown-linux-gnu'
# $RUSTFLAGS="-C target-feature=+crt-static -C link-self-contained=yes -L native=/usr/lib -l static=clang"
  # -l static=stdc++"

# x86_64-unknown-linux-gnu
# system = 'unknown-linux-gnu'

TARGET=f'{platform.machine()}-{system}'

NAME="benchmark_rust_embed_database"

$EXTRA_CXXFLAGS="-DZSTD_STATIC_LINKING_ONLY"

cargo build \
--release \
-Z build-std=std,panic_abort \
-Z build-std-features=panic_immediate_abort \
-p @(NAME) \
--target @(TARGET)

out=f"target/{TARGET}/release/{NAME}"
strip @(out)

#./sh/upx.sh
#upx --best --lzma @(out)
./@(out) | tee out.log

cat mac.md out.log > readme.md

rm out.log
