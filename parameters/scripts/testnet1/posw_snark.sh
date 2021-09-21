# This script will run the PoSW SNARK setup and move the resulting `.params`
# and `.checksum` files to `params` folder under the `src` directory.
# If the parameter size has changed, you will need to manually update these in each corresponding struct.

RUST_BACKTRACE=1 cargo run --release --example posw_snark testnet1 || exit

mv posw_snark_pk*.params ../../src/testnet1
mv posw_snark_pk.checksum ../../src/testnet1

mv posw_snark_vk.params ../../src/testnet1
mv posw_snark_vk.checksum ../../src/testnet1
