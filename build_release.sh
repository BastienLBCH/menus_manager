cargo build --release
cargo build --target=x86_64-pc-windows-gnu --release

cp target/release/menus_manager bin/menus-manager_apple_silicon
cp target/x86_64-pc-windows-gnu/release/menus_manager.exe bin/menus-manager_windows_x86_64.exe
