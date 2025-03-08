use bril_rs::{Code, EffectOps, Instruction, Program};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
struct BasicBlock {
    label: String,
    instrs: Vec<Instruction>,
    live: bool,
    in_edges: HashSet<usize>,
    out_edges: HashSet<usize>,
}

impl BasicBlock {
    fn new(s: String) -> Self {
        BasicBlock {
            label: s,
            instrs: Vec::new(),
            live: true,
            in_edges: HashSet::new(),
            out_edges: HashSet::new(),
        }
    }
}

impl fmt::Display for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.live {
            let in_set: Vec<String> = self.in_edges.iter().map(|x| x.to_string()).collect();
            let out_set: Vec<String> = self.out_edges.iter().map(|x| x.to_string()).collect();
            writeln!(
                f,
                "  {}(): in: [{}] out: [{}]",
                self.label,
                in_set.join(","),
                out_set.join(",")
            )?;
            for i in &self.instrs {
                writeln!(f, "    {}", i)?
            }
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

type BBFuns = Vec<BBFun>;

fn prog2bb(p: &Program) -> BBFuns {
    let mut funs: Vec<BBFun> = Vec::new();
    let block_ = "block_".to_string();
    for fun in &p.functions {
        let mut block_num: usize = 0;
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
                            cur_block = BasicBlock::new(block_.clone() + &block_num.to_string());
                            block_num += 1;
                        }
                        _ => cur_block.instrs.push(i.clone()),
                    },
                },
                Code::Label { label, .. } => {
                    if !cur_block.instrs.is_empty() {
                        cur_fun.blocks.push(cur_block);
                    }
                    cur_block = BasicBlock::new(label.clone());
                }
            };
        }
        cur_fun.blocks.push(cur_block);
        funs.push(cur_fun);
    }
    funs
}

fn fun2cfg(fun: &mut BBFun) {
    assert!(!fun.blocks.is_empty());
    use Instruction::*;
    let label_map = fun
        .blocks
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, bb)| {
            m.insert(bb.label.clone(), i);
            m
        });

    let mut in_edges: Vec<(usize, usize)> = Vec::new();
    let mut out_edges: Vec<(usize, usize)> = Vec::new();
    for (idx, b) in fun.blocks.iter_mut().enumerate() {
        if let Some(instr) = b.instrs.last() {
            println!("{instr:?}");
        }

        match b.instrs.last() {
            Some(Constant { .. }) | Some(Value { .. }) => {
                out_edges.push((idx + 1, idx));
            }
            Some(Effect { op, labels, .. }) => match op {
                EffectOps::Branch | EffectOps::Jump => {
                    //println!("op {op} {labels:?}");
                    for l in labels {
                        let target_idx = label_map.get(l).unwrap();
                        in_edges.push((*target_idx, idx));
                        out_edges.push((*target_idx, idx));
                    }
                }
                _ => {}
            },
            None => {}
        };
    }
    for (target, source) in in_edges {
        fun.blocks[target].in_edges.insert(source);
    }
    for (target, source) in out_edges {
        fun.blocks[source].out_edges.insert(target);
    }
}

fn main() {
    let program = bril_rs::load_program();
    let mut funs = prog2bb(&program);
    funs.iter_mut().for_each(fun2cfg);

    for f in &funs {
        print!("{}", f);
    }

    ///funs = remove_unreachable_blocks(&mut funs);

    // for f in &funs {
    //     print!("{}", f);
    // }
    println!("\nprogram here\n{}", program);
}
