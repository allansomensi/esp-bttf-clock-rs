set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

alias b := build
alias u := update

default:
    @just run

[group: 'misc']
run:
    @cargo run

# Build

[unix]
[group: 'misc']
build:
    @echo "Installing dependencies for captive_portal..."
    @npm install --prefix web/captive_portal

    @echo "Installing dependencies for web_portal..."
    @npm install --prefix web/web_portal

    @echo "Building captive_portal..."
    @npm run build --prefix web/captive_portal

    @echo "Building web_portal..."
    @npm run build --prefix web/web_portal

[windows]
[group: 'misc']
build:
    @echo "Installing dependencies for captive_portal..."
    @pushd web/captive_portal; npm install; popd

    @echo "Installing dependencies for web_portal..."
    @pushd web/web_portal; npm install; popd

    @echo "Building captive_portal..."
    @pushd web/captive_portal; npm run build; popd

    @echo "Building web_portal..."
    @pushd web/web_portal; npm run build; popd

# Update

[unix]
[group: 'misc']
update:
    @echo "Updating Rust dependencies..."
    @cargo update

    @echo "Updating captive_portal dependencies..."
    @npm update --prefix web/captive_portal

    @echo "Updating web_portal dependencies..."
    @npm update --prefix web/web_portal

[windows]
[group: 'misc']
update:
    @echo "Updating Rust dependencies..."
    @cargo update

    @echo "Updating captive_portal dependencies..."
    @pushd web/captive_portal; npm update; popd

    @echo "Updating web_portal dependencies..."
    @pushd web/web_portal; npm update; popd


[group: 'misc']
clean:
    @cargo clean

[group: 'check']
clippy:
    @cargo clippy --all --all-targets --all-features -- --deny warnings

[group: 'check']
lint:
    @cargo fmt --all -- --check
    @cargo clippy --all --all-targets -- --deny warnings

[group: 'check']
lint-fix:
    @cargo fmt --all
    @cargo clippy

[group: 'docs']
docs CRATE:
    @open "https://docs.rs/{{CRATE}}"
