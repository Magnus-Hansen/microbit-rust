microbit common og v2 er bibloteker jeg bruger til rust-microphone

rust microphone er forsøg på at få microbit med rust til at virke 

lige nu bruger microbit et python script og serial read læser fra den


for at køre rust-microphone installer:

cargo install probe-rs-tools flip-link

rustup target add thumbv7em-none-eabihf

og kør:

cargo run --release --manifest-path ./rust-microphone/rust-microphone/Cargo.toml --features v2 --target thumbv7em-none-eabihf
