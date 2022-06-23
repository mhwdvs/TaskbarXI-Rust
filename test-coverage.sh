# requires jq to be installed on your system
# https://github.com/stedolan/jq/releases

# requires rustfilt to be installed on your system
# https://github.com/luser/rustfilt

# create coverage data
cargo test >/dev/null
# process raw coverage data
llvm-profdata merge -sparse default.profraw -o default.profdata

files=""
for file in \
    $(
        cargo test --no-run --message-format=json |
            jq -r "select(.profile.test == true) | .filenames[]" |
            grep -v pdb -
    ); do
    files="$files --object $file"
done

# produce HTML report on processed coverage data
llvm-cov show \
    $files \
    --ignore-filename-regex="(cargo.registry|TaskbarXI-Rust.tests|TaskbarXI-Rust.src.main\.rs|TaskbarXI-Rust.src.lib\.rs)" \
    --instr-profile=default.profdata \
    --use-color \
    --Xdemangler=rustfilt \
    --format=html \
    --output-dir=./test-coverage-output/

# produce text summary of report that is printed to command line
llvm-cov report \
    $files \
    --ignore-filename-regex="(cargo.registry|TaskbarXI-Rust.tests|TaskbarXI-Rust.src.main\.rs|TaskbarXI-Rust.src.lib\.rs)" \
    --instr-profile=default.profdata \
    --use-color \
    --Xdemangler=rustfilt
