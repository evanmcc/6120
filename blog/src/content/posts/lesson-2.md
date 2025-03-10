---
title: "6120 Lesson 2"
description: "Writing about lesson two, wherein we work on learning the codebase and writing the basics."
date: "March 9 2025"
draft: true
---

# Lesson 2

Lesson two mostly deals with the basics of the Bril IR and how one works with it.  The construction is pleasantly unix-y, piping the various formats the IR exists in between simple transformers (there's a plain text format, but the canonical representation is in JSON).  The first actual programming step is here, outside of the task, where you're asked to write a program that consumes Bril and constructs [basic blocks](https://github.com/evanmcc/6120/commit/0adcba12616b8e1108c678a3bf62deb8fa2f1a17#diff-cb64330db7076e5a4545000fcd28a303ca64e8bd6e2b4beca7ac9fe51fafdc9d) and then a [control-flow graph](TK).
