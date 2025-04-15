for at køre rust-microphone installer:

cargo install probe-rs-tools flip-link

rustup target add thumbv7em-none-eabihf

og kør:

cargo run --release --manifest-path ./rust-microphone/rust-microphone/Cargo.toml --features v2 --target thumbv7em-none-eabihf
