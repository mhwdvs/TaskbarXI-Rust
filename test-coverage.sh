# requires jq to be installed on your system
# https://github.com/stedolan/jq/releases
cargo test
llvm-profdata merge -sparse default.profraw -o default.profdata
llvm-cov report \
    $(
        for file in \
            $(
                cargo test --no-run --message-format=json |
                    jq -r "select(.profile.test == true) | .filenames[]" |
                    grep -v pdb -
            ); do
            printf "%s %s " -object $file
        done
    ) \
    --instr-profile=default.profdata --summary-only # and/or other options
