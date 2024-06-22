# Find Project (`fp`)

A small utility to quickly change project directory on the command line.

It is heavily inspired by [telescope-project.nvim] with my theme [vim-enfocado].

[telescope-project.nvim]: https://github.com/nvim-telescope/telescope-project.nvim
[vim-enfocado]: https://github.com/wuelnerdotexe/vim-enfocado

![`fp` screenshot](assets/demo.png)

# Why

I want:

- something **easy**, **fast**, and **no setup** (or as little as possible)
- something simple overall, meaning no SQL database or tracking project navigation ...
- to search through project, not through recent directories
- possibly a [telescope.nvim]-like interface

Most other project/workspace switcher do not meet that.
It's also a good excuse to use [ratatui](https://github.com/ratatui-org/ratatui/).

[telescope.nvim]: https://github.com/nvim-telescope/telescope.nvim

# Install

```sh
cargo install --git https://github.com/Valentin271/fp
echo "alias fp='cd \$(command fp)'" >> ~/.profile # or .bash_aliases or whatever else your shell uses
```

# Functioning

Currently `fp` searches for projects in `$HOME`, ignoring hidden directories and build/dependency
directories like `node_modules`, `target`, `cmake*` and more.

## Preview

File listing is similar to running:

```sh
ls -1 -A --group-directories-first
```

It also uses your `LS_COLORS`.

# TODO

In no particular order

- [ ] implement projects cache
- [ ] configurable search path
- [ ] display `filtered entries / total` just like telescope
- [ ] configurable whitelist
- [ ] configurable blacklist?
- [ ] sort by [frecency] by default / [frecency] + search?

[frecency]: https://en.wikipedia.org/wiki/Frecency
