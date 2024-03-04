# rust_serialport_communication
-It need serialport lib in tomb file, if you don't have this crate, run:
cargo add serialport
-And add any dependences it lacked on your own.

-Better to use fast lib source of rust, such as: .cargo/config.toml in your project folder. If it doesn't exist, create a new one with the content:
----------------------------------don't copy this line-----------------------------------------
[source.crates-io] 
replace-with = 'tuna' 
  
[source.tuna] 
registry = 'https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index'
----------------------------------don't copy this line-----------------------------------------

-Build serial port:
sudo apt install socat
sudo socat -d -d pty,raw,echo=0,link=/dev/ttyVSP0 pty,raw,echo=0,link=/dev/ttyVSP4 &

-Change the mode of serial port to support cargo run and vscode debug:
sudo chmod a+rw /dev/ttyVSP0
sudo chmod a+rw /dev/ttyVSP4

-Compile：
cd /ue
cargo build

-Run:
cargo run
