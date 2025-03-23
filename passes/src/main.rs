use clap::Parser;

mod blocks;
use crate::blocks::{remove_unreachable_blocks, *};

mod dce;
use crate::dce::dce;

mod lvn;
use crate::lvn::lvn_dce;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "PASSES")]
    passes: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

type Pass = fn(&mut BBFun);

fn main() {
    let cli = Cli::parse();
    let all_passes: Vec<Pass> = vec![remove_unreachable_blocks, dce, lvn_dce];

    let mut passes: Vec<Pass> = Vec::new();
    let mut debug: bool = cli.debug > 0;
    if let Some(pass_str) = cli.passes {
        if pass_str == "all".to_string() {
            passes.extend(all_passes);
        } else {
            for p in pass_str.split(",") {
                match p {
                    "dce" => passes.push(dce),
                    "lvn_dce" => passes.push(lvn_dce),
                    "remove_unreachable_blocks" => passes.push(remove_unreachable_blocks),
                    _ => panic!("unknown pass: {}", p),
                }
            }
        }
    } else {
        passes.extend(all_passes);
    }

    let program = bril_rs::load_program();
    let mut funs = prog2bb(&program);
    funs.iter_mut().for_each(fun2cfg);

    if debug {
        println!("initial program\n{}", program);
    }

    for pass in passes {
        funs.iter_mut().for_each(pass)
    }

    if debug {
        println!("optimized cfg");
        for f in &funs {
            print!("{}", f);
        }
    } else {
        let program2 = cfg2program(&funs);
        bril_rs::output_program(&program2);
    }
}
