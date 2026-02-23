# Symlink builder  
  
Symbolic link for Windows without `.lnk` and capacity to pass arguments  
  
## How to use  
1. Build the binary  
2. Rename the binary as you wish, e.g., `<binary-name>.exe`  
3. Put the binary and `<binary-name>.toml` into the same folder  
4. The binary will run the `source` section of `<binary-name>.toml` when invoked  

## Usage Example

~~_Somewhat Pseudocode for Windows_~~

#### Setup
```bash
cargo build --release
cp target/release/symlink-builder.exe $HOME/.local/bin/l.exe
printf 'source = "C:/msys2/usr/bin/ls.exe --color=tty -olah"\n' > $HOME/.local/bin/l.toml
```
#### Run
```bash
l.exe
```
#### Output
```bash
total 27K
drwxr-xr-x 1 ParkSnoopy    0 Feb 24 03:21 .
drwxr-xr-x 1 ParkSnoopy    0 Feb 24 03:08 ..
drwxr-xr-x 1 ParkSnoopy    0 Feb 24 03:34 .git
-rw-r--r-- 1 ParkSnoopy    9 Nov 29 03:41 .gitignore
-rw-r--r-- 1 ParkSnoopy 3.0K Feb 24 03:37 Cargo.lock
-rw-r--r-- 1 ParkSnoopy  248 Feb 24 03:37 Cargo.toml
-rw-r--r-- 1 ParkSnoopy 1.1K Nov 29 03:41 LICENSE
-rw-r--r-- 1 ParkSnoopy 1.3K Feb 24 03:32 README.md
-rw-r--r-- 1 ParkSnoopy   54 Nov 29 03:41 l.toml
drwxr-xr-x 1 ParkSnoopy    0 Feb 20 18:08 src
drwxr-xr-x 1 ParkSnoopy    0 Feb 24 03:10 target
```
