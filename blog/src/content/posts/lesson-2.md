---
title: "6120 Lesson 2"
description: "Writing about lesson two, wherein we work on learning the codebase and writing the basics."
date: "March 9 2025"
---

Lesson two mostly deals with the basics of the Bril IR and how one works with it.  The construction is pleasantly unix-y, piping the various formats the IR exists in between simple transformers (there's a plain text format, but the canonical representation is in JSON).  The first actual programming step is here, outside of the task, where you're asked to write a program that consumes Bril and constructs [basic blocks](https://github.com/evanmcc/6120/commit/0adcba12616b8e1108c678a3bf62deb8fa2f1a17) and then a [control-flow graph](https://github.com/evanmcc/6120/commit/21c3d168ceb74f52c4c05b5ffecb93c1a54a4dad).

I think the thing that surprised me here was the simplicity of these operations.  I had for some reason thought that creating basic blocks was complicated, and that CFGs were hard to construct, but the most basic versions are a few lines apiece.  Later, I suspect that I'll have more to say about the course material, but this is very much just starting to get your feet wet.

```rust
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
                    cur_block = BasicBlock::new(block_.clone() +
                                                &block_num.to_string());
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
```

My rust remains pretty primitive (please forgive all the `clones()`), but I do like that you can chain tests together in a case with `|`, which is something that other languages could stand to add.  You might note in my initial passes that threw away a lot of information that I had to go back and re-add later, when I needed to actually pass the program back to standard out, instead of printing my cut-down versions.  Rust was pretty nice for this, I just changed the output type and then followed the errors back through the code fixing type errors as I went, and then at the end it worked again, with the new functionality in place, and nothing broken.  I realize that this is pretty standard for typed languages, but Rust's errors continue to be top notch.

## Tasks

The theme this lesson is getting used to [Bril](https://capra.cs.cornell.edu/bril/intro.html) and various tools for working with it.

- [ ] Write a new benchmark.
  - I didn't do this!  Since I'm the one who's both grading and taking the course, I feel OK skipping it.  I might come back to it later, it's probably a good idea to do something like this, but since there's more benchmarking in a lesson or two, I thought that it might be more interesting to do it in a broader context.
- [x]  Write a program to analyze or transform Bril programs in some other small way that you invent. 
  - I wrote a little pass that removes basic blocks that have no pointers to them [here](https://github.com/evanmcc/6120/commit/5ebd86507365845eb4865e3aa318de51e7dfbd33#diff-cb64330db7076e5a4545000fcd28a303ca64e8bd6e2b4beca7ac9fe51fafdc9dR151-R158). 
- [x]  Use Turnt to test your new tool. Think carefully about what makes a good test for your tool, no matter how trivial.
  - I wrote a turnt test that tests the above (size) optimization pass [here](https://github.com/evanmcc/6120/commit/2c7f197816f6aed8bc8b58843091c2d2dd2a23be).
- [x] Implement the algorithms to form basic blocks and build a control flow graph.
  - See the links and discussions above.
