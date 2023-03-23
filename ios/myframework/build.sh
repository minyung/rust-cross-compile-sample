set -e

RUST_LIB_NAME="librustlib.a"

INTEL_CPU_SWIFTLINT_PATH="/usr/local/bin"
ARM_CPU_SWIFTLINT_PATH="/opt/homebrew/bin"
PATH="$PATH:$INTEL_CPU_SWIFTLINT_PATH:$ARM_CPU_SWIFTLINT_PATH"

if which swiftlint > /dev/null; then
    swiftlint
else
    echo "error: swiftlint not installed, download from https://github.com/realm/SwiftLint"
    echo "  - brew install swiftlint"
    exit 1
fi

BUILD_PHASE=""
CARGO_BUILD_OPTION="--features=ios"

if [ "${CONFIGURATION}" = "Debug" ]; then
    BUILD_PHASE="debug"
else
    BUILD_PHASE
    CARGO_BUILD_OPTION="$CARGO_BUILD_OPTION --release"
fi

CARGO_PATH="$HOME/.cargo/bin"

# use /usr/bin/cc, not use xcode's cc
PATH="/usr/bin:$PATH:$CARGO_PATH"

RUST_PROJ_PATH="$PROJECT_DIR/../../rust"

LIBRUST_PATH="$PROJECT_DIR/librust"
RUST_HEADER_OUTPUT_PATH="$LIBRUST_PATH/rust.h"
STATIC_RUST_LIB_PATH="$LIBRUST_PATH/$RUST_LIB_NAME"

#################### main ######################

echo "RUST_PROJ_PATH : $RUST_PROJ_PATH"
cd "$RUST_PROJ_PATH"

# Generate C bindings
if [ -f "$RUST_HEADER_OUTPUT_PATH" ]; then
    rm "$RUST_HEADER_OUTPUT_PATH"
fi

if which cbindgen > /dev/null; then
    cbindgen -l C -o "$RUST_HEADER_OUTPUT_PATH"
else
    echo "  - cargo install --force cbindgen"
    exit 1
fi

if [ "$EFFECTIVE_PLATFORM_NAME" = "-iphonesimulator" ]; then
    cargo build -v --target aarch64-apple-ios-sim $CARGO_BUILD_OPTION
    cargo build -v --target x86_64-apple-ios $CARGO_BUILD_OPTION
    lipo -create "target/aarch64-apple-ios-sim/$BUILD_PHASE/$RUST_LIB_NAME" "target/x86_64-apple-ios/$BUILD_PHASE/$RUST_LIB_NAME" -output "target/$RUST_LIB_NAME"
else
    cargo build -v --target aarch64-apple-ios $CARGO_BUILD_OPTION
    cp "target/aarch64-apple-ios/$BUILD_PHASE/$RUST_LIB_NAME" "target/$RUST_LIB_NAME"
fi

cp "./target/$RUST_LIB_NAME" "$STATIC_RUST_LIB_PATH"
