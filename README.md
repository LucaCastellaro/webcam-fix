# webcam-fix
Fix webcam problems for ThinkPad x280 running Ubuntu.

## Add to .zshrc
For a quick & easy usage, you can add the "webcam-fix" command to your .zshrc:
1. Build the project in release mode: `cargo build --release`
2. Enter **target/release**, here you should have a file called **webcam-fix**. Copy the full path of this file
3. Open your **.zshrc**, at the end add the following: `alias webcam-fix="sudo full/path/to/webcam-fix"`
4. Run `source .zshrc` to reload you configuration
5. Now you can run the command `webcam-fix` in your terminal
