# 3dge

3dge is a 3d graphics engine that aims to more or less let you render things
with raw wgpu while hiding tedium behind a ultra low cost abstraction.


I couldn't get the `edge` name in crates.io, so unfortunately I had to get
creative with the crate name. If you want to give this bad boy a whirl, add
this to your `Cargo.toml`

```

```

That hurts my eyes to look at, so since I don't use the actual (lovely looking)
edge crate I usually just cheat and do this:

```

```

The goal of this project is to be simple by default, with the ability to open up
the hood if need be. Here's what I mean:

```
```

The main thing

This probably looks pretty high-level at first, so bear with me and I'll sow you what I mean 

? Here's what's actually happening under the hood:

  1. 

graphics engine. I'm mostly working on it to experiment with procedural
generation, physics simulation, and general data visualization tasks.
