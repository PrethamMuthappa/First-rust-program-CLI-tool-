use clap::Parser;
#[derive(Parser, Debug)]
#[command
  (author="pretham",
  version="1.0",
  about="simple temperature converter",
  long_about = None)
]
struct Args {
    #[arg(short('F'),long,default_value_t=false)]
    iffaren:bool,

    #[arg(short, long, default_value_t=0.0)]
    celsius: f32,

    #[arg(short('f'),long,default_value_t=0.0)]
    fare:f32,

}
impl Args {
    fn con(&self)->f32 {
        (self.celsius*9.0/5.0)+32.0
    }

    fn far(&self)->f32 {
        self.fare-32.0*5.0/9.0
    }
}

fn main() {
    let args = Args::parse();
    let cc=args.con();
    let ff = args.far();

    if !args.iffaren {
        println!("celsius is {:?} converted to {:?}F", args.celsius, cc)
    }
    else {
        println!("farenhite is {:?} converted to {:?}C", args.fare, ff)
    }

}
