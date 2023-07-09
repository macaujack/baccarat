trunk build --release baccarat_assistant/index.html
cargo build --release --features "embed_website_assets" --bin baccarat_solver_service
echo
echo
echo "If everything works fine, you can find the executable (named 'baccarat_solver_service' on *nix or 'baccarat_solver_service.exe' on Windows) under the path target/release/"
