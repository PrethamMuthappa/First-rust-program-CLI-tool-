use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    #[arg(short('f'),long)]
    iffaren:bool,

    #[arg(short, long)]
    celsious: f32,

    #[arg(short('Z'),long)]
    fare:f32,

}
impl Args {
    fn con(&self)->f32 {
        (self.celsious*9.0/5.0)+32.0
    }

    fn far(&self)->f32 {
        self.fare-32.0*5.0/9.0
    }
}

fn main() {
    let args = Args::parse();

    let abc=args.con();
    let abd = args.far();

    if !args.iffaren {
        println!("celsious is {:?} converted to {:?}F", args.celsious, abc)
    }
    else {
        println!("farenhite is {:?} converted to {:?}C", args.fare, abd)
    }

}