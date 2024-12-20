name := 'cosmic-iio'

base-dir := '/usr'
bin-src := 'target/release' / name
bin-dst := base-dir / 'bin' / name

service-file := name + '.service'
service-dst := '/etc/systemd/user' / service-file

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
    cp {{service-file}} {{service-dst}}

# Run as regular user
start:
    systemctl --user enable {{service-file}}
    systemctl --user start {{service-file}}

# Run as regular user
stop:
    systemctl --user stop {{service-file}}
    systemctl --user disable {{service-file}}

# Requires sudo
uninstall:
    rm {{bin-dst}}
    rm {{service-dst}}

