use bril_rs::{Code, EffectOps, Instruction};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
struct BasicBlock {
    label: String,
    instrs: Vec<Instruction>,
    in_edges: HashSet<usize>,
    out_edges: HashSet<usize>,
}

impl BasicBlock {
    fn new(s: String) -> Self {
        BasicBlock {
            label: s,
            instrs: Vec::new(),
            in_edges: HashSet::new(),
            out_edges: HashSet::new(),
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  {}():", self.label)?;
        for i in &self.instrs {
            writeln!(f, "    {}", i)?
        }
        Ok(())
    }
}

struct BBFun {
    name: String,
    blocks: Vec<BasicBlock>,
}

impl BBFun {
    fn new(s: String) -> Self {
        BBFun {
            name: s,
            blocks: Vec::new(),
        }
    }
}

impl fmt::Display for BBFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {{", self.name)?;
        for b in &self.blocks {
            write!(f, "{}", b)?
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

fn main() {
    let program = bril_rs::load_program();
    let mut funs: Vec<BBFun> = Vec::new();
    for fun in &program.functions {
        let mut cur_fun = BBFun::new(fun.name.clone());
        let mut cur_block = BasicBlock::new("start_".to_string() + &fun.name);
        for instr in &fun.instrs {
            use Instruction::*;
            match instr {
                Code::Instruction(i) => match i {
                    Constant { .. } => cur_block.instrs.push(i.clone()),
                    Value { .. } => cur_block.instrs.push(i.clone()),
                    Effect { op, .. } => match op {
                        EffectOps::Branch | EffectOps::Jump => {
                            cur_block.instrs.push(i.clone());
                            cur_fun.blocks.push(cur_block.clone());
                            cur_block = BasicBlock::new("".to_string());
                        }
                        _ => cur_block.instrs.push(i.clone()),
                    },
                },
                Code::Label { label, .. } => {
                    cur_fun.blocks.push(cur_block);
                    cur_block = BasicBlock::new(label.clone());
                }
            };
        }
        cur_fun.blocks.push(cur_block); //double adding sometimes?
        funs.push(cur_fun);
    }

    for f in funs {
        print!("{}", f);
    }
    println!("\nprogram here\n{}", program);
}
