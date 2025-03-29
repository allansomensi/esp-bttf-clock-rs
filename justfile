default:
    @just run

run:
    @cargo run

[group: 'misc']
build:
    @echo "Installing dependencies for captive_portal..."
    @npm install --prefix web/captive_portal/
    @echo "Installing dependencies for web_portal..."
    @npm install --prefix web/web_portal/
    @echo "Building captive_portal..."
    @npm run build --prefix web/captive_portal/
    @echo "Building web_portal..."
    @npm run build --prefix web/web_portal/

[group: 'misc']
update:
    @echo "Updating Rust dependencies..."
    @cargo update
    @echo "Updating captive_portal dependencies..."
    @npm update --prefix web/captive_portal/
    @echo "Updating web_portal dependencies..."
    @npm update --prefix web/web_portal/

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

[group: 'misc']
clean:
    @cargo clean
