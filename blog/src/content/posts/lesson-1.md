---
title: "6120 Lesson 1"
description: "Some initial throat clearing about motivations."
date: "March 2 2025"
---

Welcome to the single-serving blog I am keeping to document my self-guided pass through [Cornell's cs6120](https://www.cs.cornell.edu/courses/cs6120), taught by [Adrian Sampson](https://www.cs.cornell.edu/~asampson/).  It is being done in early 2025, so may not be reflective of your experience with the course if it happens in some other year.

Hopefully these blog posts can be useful for other self-guided learners and to the course instructor in making the course more enriching for other people who take the course on their own.  I would encourage anyone who takes the course to email me and I will add your links to blogs and repos here.

## Why 6120?

Firstly, it's available.  The instructor has put most of the course materials online (only the class-specific grading and zulip discussions are not available).  I ran into it because of [Max Bernstein](https://bernsteinbear.com/) boosting a [mastodon post](https://mastodon.social/@adrian@discuss.systems/114065548299028516) from the author (he also has a good [SSA resource page](https://bernsteinbear.com/blog/ssa/). I had run into Bril before and thought that it was an interesting idea, but I simply didn't realize that the whole course was online and available!

What makes this most interesting to me is that most of my compiler efforts in the past have stalled out on the problems that Bril and the course are meant to solve: parsers being something that no one loves to write, a large body of code for any new language has to be self-generated, reference implementations have to be written, and etc.  One needs to build the world first upon which to set up one's first tent.  While it's possible, it's not easy for someone with a lot of other demands on their time, and having all of the work done for you, so that you can focus on the interesting work and the important learning.  Instead of weeks of writing parsers and example programs, you can get right into basic block and CFG construction within a few minutes of starting to code.  And so, we begin!

## About me

I am a fairly senior programmer with a long, non-compiler career, mostly focusing on distributed databases and similar systems, with a side-focus on performance work.  I have a love of programming languages, as they're the most flexible tools that we have, and I enjoy that their various affordances give rise to so many different styles of programming.

## Tasks

For this lesson, there aren't really any tasks that are relevant to the self-guided version, but I will take the Zulip prompt, "Mention a compilers topic you’d like to learn about someday, either in this class or beyond".  Abstract interpretation is the topic in compilers that interests me most, because it feels like automating how I think about program analysis as a programmer, so that the passes make more sense to me.  My hope for the semester project is to write some abstract analyses that apply to automatic memory management, another major interest.

Another task to pick a paper to read and lead the discussion of.  Given my aforementioned interest in abstract interpretation, at some point here I will do a longer blog post or series of posts on [this paper](http://codex.top/assets/publications/pdfs/2024-pldi-compiling-with-abstract-interpretation-with-appendices.pdf), which hints at a potentially simpler construction for some compilers.
