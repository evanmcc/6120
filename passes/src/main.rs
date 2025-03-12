use bril_rs::{Code, EffectOps, Function, Instruction, Program};
use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fmt;
//use std::process;

#[derive(Debug, Clone)]
struct BasicBlock {
    label: String,
    label_instr: Option<Code>,
    instrs: Vec<Code>,
    live: bool,
    in_edges: HashSet<usize>,
    out_edges: HashSet<usize>,
}

impl BasicBlock {
    fn new(s: String) -> Self {
        BasicBlock {
            label: s,
            label_instr: None,
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
    blocks: Vec<BasicBlock>,
    function: Function,
}

impl BBFun {
    fn new(f: Function) -> Self {
        BBFun {
            blocks: Vec::new(),
            function: f,
        }
    }
}

impl fmt::Display for BBFun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {{", self.function.name)?;
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
        let mut cur_fun = BBFun::new(fun.clone());
        let mut cur_block = BasicBlock::new("start_".to_string() + &fun.name);
        for instr in &fun.instrs {
            use Instruction::*;
            match instr {
                Code::Instruction(i) => match i {
                    Constant { .. } => cur_block.instrs.push(instr.clone()),
                    Value { .. } => cur_block.instrs.push(instr.clone()),
                    Effect { op, .. } => match op {
                        EffectOps::Branch | EffectOps::Jump => {
                            cur_block.instrs.push(instr.clone());
                            cur_fun.blocks.push(cur_block.clone());
                            cur_block = BasicBlock::new(block_.clone() + &block_num.to_string());
                            block_num += 1;
                        }
                        _ => cur_block.instrs.push(instr.clone()),
                    },
                },
                Code::Label { label, .. } => {
                    if !cur_block.instrs.is_empty() {
                        cur_fun.blocks.push(cur_block);
                    }
                    cur_block = BasicBlock::new(label.clone());
                    cur_block.label_instr = Some(instr.clone());
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
        if let Some(c) = b.instrs.last() {
            match c {
                Code::Instruction(Constant { .. }) | Code::Instruction(Value { .. }) => {
                    out_edges.push((idx + 1, idx));
                }
                Code::Instruction(Effect { op, labels, .. }) => match op {
                    EffectOps::Branch | EffectOps::Jump => {
                        //println!("op {op} {labels:?}");
                        for l in labels {
                            let target_idx = label_map.get(l).unwrap();
                            in_edges.push((*target_idx, idx));
                            out_edges.push((*target_idx, idx));
                        }
                    }
                    EffectOps::Return => {}
                    _ => out_edges.push((idx + 1, idx)),
                },
                Code::Label { .. } => unreachable!(),
            };
        }
    }
    for (target, source) in in_edges {
        fun.blocks[target].in_edges.insert(source);
    }
    for (target, source) in out_edges {
        fun.blocks[source].out_edges.insert(target);
    }
}

fn remove_unreachable_blocks(fun: &mut BBFun) {
    let mut changed = true;
    while changed {
        changed = false;
        let mut acc = HashSet::new();
        for (idx, block) in &mut fun.blocks.iter_mut().enumerate() {
            if idx != 0 && block.live && block.in_edges.is_empty() {
                block.live = false;
                changed = true;
                acc.insert(idx);
            }
        }
        // we have to remove the dead block from all in sets otherwise the fix point iteration won't
        // make progress
        for block in &mut fun.blocks.iter_mut() {
            let _ = block.in_edges.difference(&acc);
        }
    }
}

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
        for (_name, pos) in &defs {
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

fn flatten_basic_blocks(bbs: &Vec<BasicBlock>) -> Vec<Code> {
    let mut ret = Vec::new();

    for bb in bbs {
        if let Some(label) = &bb.label_instr {
            ret.push(label.clone());
        }
        if bb.live {
            ret.extend(
                bb.instrs
                    .clone()
                    .into_iter()
                    .to_owned()
                    .filter(|c| not_nop(c)),
            );
        }
    }
    ret
}

fn not_nop(c: &Code) -> bool {
    match c {
        Code::Instruction(Instruction::Effect { op, .. }) => {
            if let EffectOps::Nop = op {
                false
            } else {
                true
            }
        }
        _ => true,
    }
}

fn cfg2program(funs: &Vec<BBFun>) -> Program {
    let mut ret = Program {
        functions: Vec::new(),
    };
    for fun in funs {
        let instrs = flatten_basic_blocks(&fun.blocks);
        let f = Function {
            args: fun.function.args.clone(),
            instrs,
            name: fun.function.name.clone(),
            pos: fun.function.pos.clone(),
            return_type: fun.function.return_type.clone(),
        };
        ret.functions.push(f);
    }
    ret
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
