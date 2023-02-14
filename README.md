# New terminal hyprland
A simple utility to spawn new terminals in an intelligant way, new terminals
will be spawned in the same directory as the active window.  
If the active window contains a zellij session then it will create a new pane
for that session

## Building and running


Install with cargo
Build
```fish
git clone https://github.com/ElSargo/new-terminal-hyprland
cd new-termainal-hyprland
```
Try it
```fish
cargo run
```

Install
```fish
cargo install --path ./
```


You can of course invoke it from the command line but I have it bound to
SUPER+RETURN in hyprland.conf

### Supported terminals
* Alacritty  

I may add more in future