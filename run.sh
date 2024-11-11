
# a json file is created for each task in the script folder
# memory usage measurement (measure proving only)
# time -v output is not logged in a json file, to do so,
# optional: log into a text file with > sp1_ecdsa_mem at the end of command or use nohup/tmux 

# sp1 
cd sp1/merkle
cd program/
cargo prove build
cd ../script/
RUST_LOG=info cargo run --release -- --task merkle
/usr/bin/time -v cargo run --release -- --task merkle --prove-only 
cd ../../

cd ecdsa
cd program/
cargo prove build
cd ../script/
RUST_LOG=info cargo run --release -- --task ecdsa
/usr/bin/time -v cargo run --release -- --task ecdsa --prove-only 
cd ../../


cd ml
cd program/
cargo prove build
cd ../script/
RUST_LOG=info cargo run --release -- --task mnist
RUST_LOG=info cargo run --release -- --task dlrm
# For memory usage
/usr/bin/time -v cargo run --release -- --task mnist --prove-only 
/usr/bin/time -v cargo run --release -- --task dlrm --prove-only 
cd ../../../


# risc0
cd risc0/merkle/
RUST_LOG=info cargo run --release -- --task merkle
/usr/bin/time -v cargo run --release -- --task merkle --prove-only 
cd ..
cd ecdsa/
RUST_LOG=info cargo run --release -- --task ecdsa
/usr/bin/time -v cargo run --release -- --task ecdsa --prove-only 
cd ..
cd ml/
RUST_LOG=info cargo run --release -- --task mnist
/usr/bin/time -v cargo run --release -- --task mnist --prove-only 
RUST_LOG=info cargo run --release -- --task dlrm
/usr/bin/time -v cargo run --release -- --task dlrm --prove-only 
cd ../../