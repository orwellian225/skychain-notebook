# Skychain Notebook Data Model

The skychain model is slightly different to the jupyter structure.
Instead of writing everything in a single file, skychain notebooks create seperate page files.
These page fails are easily navigable and make it possible to maintain code in a more sustainable model.

The goal is to make a _book_ rather then a single page of content.

1. Notebook
    * `iscnb` - interactive skychain notebook
    * A notebook is the top level structure.
    * It can have cells, but the intendended purpose of these cells is information & discussion.
2. Chapter
    * `chapter_title/`
    * A directory within a notebook
    * Organize files into a more succient manner
3. Page
    * `iscpg` - interactive skychain page
    * A page of code and writing
4. Cell
    * A single statically typed cell of data

## Kernels

## Themes

## Config

## Cache

## Notebook Directory Structure

```txt
notebook_dir
├── .kernel
│   ├── <<bash kernel>>
│   └── <<python kernel>>
├── .themes
│   ├── <<default theme>>
│   └── <<user theme>>
├── .config
│   └── <<local config>>
├── .cache
│   └── <<>>
├── main.iscnb
├── page_one.iscpg
├── chapter_one
│   ├── page_one.iscpg
│   └── page_two.iscpg
└── chapter_two
    ├── page_one.iscpg
    └── page_two.iscpg
```