# Smart Paste
This application allows pasting into any destination (even citrix). This is possible by sending the keystrokes in quick succession.
After starting the application the hotkey "ctrl + shift + v" can be used to automatically paste the last entry in the clipboard.

> :warning: **When copying multiple lines**: Return key is sent after every line, which can lead to issues in single-line inputs!

## Installation
To install the application it has to be built using cargo.
1. Install Rust and Cargo via the official documentation (https://www.rust-lang.org/tools/install)
2. Run `cargo build --release`
3. The executable for your OS will be located in ./target/release/

The application can be manually executed by executing the binary build or automatically started via the steps below.

## MacOS Privacy Settings (Required)
MacOS blocks external control of keystrokes via the settings. This app can be added to the whitelist in the settings.
1. Open Settings > Privacy & Security > Accessibility
2. Add Terminal (or smart-paste if running as Application)

## MacOS Autostart (Optional)
To automatically start the app on MacOS you have to create an application via the automator and then add it to the Login items.
1. Move the executable to `/usr/local/bin/smart-paste`
2. Press "Command + Space" - search for "automator"
3. Choose Application
4. Search for "shell script" and select "Run Shell Script"
5. As command enter `/usr/local/bin/smart-paste`
6. Press "Command + S" to save the app under the name `smart-paste`

After creating the Application you now have to add it to the Login items:
1. Open Settings > General > Login Items
2. Click the "+" button
3. Select the created app (smart-paste)

## Windows Autostart (Optional)
To add the app to autostart for windows it should be possible to simply add the application to the startup folder.
1. Open the path `%userprofile%\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`
2. Paste the executable into this directory
