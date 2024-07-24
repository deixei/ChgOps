# ChgOps
Change management (Chg) and Operations (Ops)

## Developing

Install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

### local testing
cargo run -- run --help

cargo run -- run -n playbook -p ./playbooks/workspace2 -a STAGE=dev

#### build

cargo build

After build ~/repos/github/deixei/ChgOps/target/debug# ./chgops run --help

##### debug

https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/


## Author

[Marcio Parente](https://github.com/deixei) from deixei.com

To understand the overall context of this project read this book: [ENTERPRISE SOFTWARE DELIVERY: A ROADMAP FOR THE FUTURE](https://www.amazon.de/-/en/Marcio-Parente/dp/B0CXTJZJ2X/)