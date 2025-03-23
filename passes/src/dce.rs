use crate::blocks::*;
use bril_rs::{Code, EffectOps, Instruction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Position {
    block: usize,
    instr: usize,
}

pub fn dce(fun: &mut BBFun) {
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
