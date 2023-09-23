# First-rust-program
Wrote my first program in rust using clap library which is used for argument parsing </br>
This program can convert temperature from celsius to farenheit and vice versa through arguments from your terminal<br>

`cargo run -- -help `
```
simple temperature converter
Usage: clap [OPTIONS]

Options:
  -F, --iffaren              
  -c, --celsius <CELSIUS>  
  -f, --farenhite <FARE>          
  -h, --help                 Print help
  -V, --version              Print version

```
by defaut the program will convert from celsius to  farenhite , if u want to do vise versa then u must inclued the -F flag in the argument
`cargo run -- -F -f 12`
