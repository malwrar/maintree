# maintree

I like using git (and most of the world seems to as well), but one repo per
project can be a rather awkward fit. Most of the code I write isn't complete
enough to warrant the commitment of a separate repo, but I still often find
myself wanting to use it in multiple places (or at least share it with others!)

So, hear me out--why shouldn't I just put most of my code in one repo?

I know some of you internet people like to sound smart shitting on monorepos by
accusing their proponents of cargo-culting google or whatever. Think about it
though, doesn't most of the stuff you work on end up in a few buckets?

1. Common snippets or pillaged bits of other projects.
2. Abstraction you develop for one project, but then want to extend for a
   different project later.
3. Cool experiment with some random framework.
4. Non-code assets with no clear owner or build system.
5. Code split across multiple locations and difficult to find or use with
   editor autocompletion.

I at least find that 99% of the time one or more of the above is true, and so
this repo is an attempt to solve all of those problems at once by putting
everything in one place under one build system (and all it costs is a little
bit of bazel wrangling!)


## map

Brief summary of what you'll find in this repo:

- [common](./common): Highly reusable code that isn't complete enough to warrant
  its own library/crate/whatever.
- [wip](./wip): Random ideas in varying states of completion.
- [viz](./viz): Various graphviz, plantuml, etc diagrams.
- [models](./models): 3d models (git lfs recommended)
- [random](./random): Ideas that don't have a good category, like experimenting
  with interesting libraries or a trying out a new database.
- [trash](./trash): Stuff that takes up space but I don't want to get rid of.


## building stuff

*WARNING*: bazel seems to break with jdk16+, I suggest using jdk11 if you want
to build anything here.

Simple command to list all the targets that can be built:

```shell
pwd  # make sure you're in the maintree root dir
bazel query ...
```


## contributing

**Be sure to make your changes based on the `stable` branch**
([convenient github link][]https://github.com/malwrar/maintree/tree/stable).
`unstable` is the default branch and is where I do most of my development. I'd
change it, but then my github wouldn't look as pretty to casual browsers
:wink:).

If you manage to build any of the projects in this repo and want to share any
changes and/or additions you've made, I'm happy to accept them via pull request
here on Github!
