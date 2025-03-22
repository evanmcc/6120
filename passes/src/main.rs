use bril_rs::{Code, EffectOps, Function, Instruction, Program};
use std::collections::HashMap;
use std::env::args;
//use std::process;

mod blocks;
use crate::blocks::*;

#[derive(Debug, Clone)]
struct Position {
    block: usize,
    instr: usize,
}

fn dce(fun: &mut BBFun) {
    let mut changed = true;
    let mut acc: Vec<Position> = Vec::new();
    while changed {
        changed = false;
        let mut defs: HashMap<String, Position> = HashMap::new();
        for (block_idx, block) in fun.blocks.iter().enumerate() {
            for (instr_idx, instr) in block.instrs.iter().enumerate() {
                match instr {
                    Code::Label { .. } => {}
                    Code::Instruction(Instruction::Value { dest, args, .. }) => {
                        //this is a type of use
                        for arg in args {
                            let _ = defs.remove(arg);
                        }
                        if let Some(old_def) = defs.insert(
                            dest.clone(),
                            Position {
                                block: block_idx,
                                instr: instr_idx,
                            },
                        ) {
                            acc.push(old_def);
                        };
                    }
                    Code::Instruction(Instruction::Constant { dest, .. }) => {
                        if let Some(old_def) = defs.insert(
                            dest.clone(),
                            Position {
                                block: block_idx,
                                instr: instr_idx,
                            },
                        ) {
                            acc.push(old_def);
                        };
                    }
                    Code::Instruction(Instruction::Effect { args, .. }) => {
                        //this is a type of use
                        //eprintln!("eff: {:?}", instr)
                        for arg in args {
                            let _ = defs.remove(arg);
                        }
                    }
                }
            }
        }
        //any unused defs are emptied into acc
        for pos in defs.values() {
            acc.push(pos.clone());
        }
        defs.clear();
        //eprintln!("acc {:?}", acc);
        if !acc.is_empty() {
            changed = true;
            //for speed, replace dead instructions with nops for later filtering
            for Position { block, instr } in &acc {
                fun.blocks[*block].instrs[*instr] = Code::Instruction(Instruction::Effect {
                    args: vec![],
                    funcs: vec![],
                    labels: vec![],
                    op: EffectOps::Nop,
                    pos: None,
                });
            }
            acc.clear();
        }
    }
}

fn main() {
    let args = args();
    let mut debug: bool = false;
    for arg in args {
        if arg.starts_with("/") {
            continue;
        }
        match arg.as_str() {
            "-d" => debug = true,
            _ => {
                eprintln!("unknown arg: {}", arg);
                //process::exit(5);
            }
        }
    }

    let program = bril_rs::load_program();
    let mut funs = prog2bb(&program);
    funs.iter_mut().for_each(fun2cfg);

    if debug {
        println!("initial program\n{}", program);
    }
    let mut passes: Vec<fn(&mut BBFun)> = vec![remove_unreachable_blocks];
    passes.push(dce);

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
