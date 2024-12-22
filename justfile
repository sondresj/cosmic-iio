name := 'cosmic-iio'

bin-src := 'target/release' / name
bin-dst := '/usr/bin' / name

service := name + '.service'
service-dst := '/etc/systemd/user' / service

default: run

clean:
    cargo clean

check:
    cargo clippy --all-features -- -W clippy::pedantic

_build *args:
    cargo build {{args}}

build-debug *args: (_build args)
build-release *args: (_build '--release' args)

test *args:
    cargo test {{args}}

run *args:
    cargo run {{args}}

# Requires sudo
install:
    install -Dm0755 {{bin-src}} {{bin-dst}}
    mkdir -p '/etc/systemd/user'
    cp {{service}} {{service-dst}}

# Run as regular user
start:
    systemctl --user enable {{service}}
    systemctl --user start {{service}}

# Run as regular user
stop:
    systemctl --user stop {{service}}
    systemctl --user disable {{service}}

# Requires sudo
uninstall:
    rm {{bin-dst}}
    rm {{service-dst}}

