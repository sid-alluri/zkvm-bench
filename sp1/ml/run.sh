# to run
RUST_LOG=info cargo run --release -- --task dlrm
RUST_LOG=info cargo run --release -- --task mnist
# For memory usage, time -v might not be same on all machines
/usr/bin/time -v RUST_LOG=info cargo run --release -- --task dlrm --prove_only 
/usr/bin/time -v RUST_LOG=info cargo run --release -- --task mnist --prove_only 