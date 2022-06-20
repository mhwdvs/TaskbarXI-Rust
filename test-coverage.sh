# requires jq to be installed on your system
# https://github.com/stedolan/jq/releases

# create coverage data
cargo test
# process raw coverage data
llvm-profdata merge -sparse default.profraw -o default.profdata
# produce report on processed coverage data
llvm-cov report \
    $(
        for file in \
            $(
                cargo test --no-run --message-format=json |
                    jq -r "select(.profile.test == true) | .filenames[]" |
                    grep -v pdb -
            ); do
            printf "%s %s " --object $file
        done
    ) \
    --ignore-filename-regex="(cargo.registry|TaskbarXI-Rust.tests|TaskbarXI-Rust.src.main\.rs|TaskbarXI-Rust.src.lib\.rs)" --instr-profile=default.profdata #--show-functions

#$(
#    find $(pwd) -wholename "*/src/*.rs"
#) \

#llvm-cov show \
#    $(
#        for file in \
#            $(
#                cargo test --no-run --message-format=json |
#                    jq -r "select(.profile.test == true) | .filenames[]" |
#                    grep -v pdb -
#            ); do
#            printf "%s %s " --object $file
#        done
#    ) \
#    --ignore-filename-regex="(.cargo/registry|./tests/|./src/main.rs|./src/lib.rs)" --instr-profile=default.profdata
