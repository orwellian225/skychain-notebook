# Skychain Notebook Data Model

The skychain model is slightly different to the jupyter structure.
Instead of writing everything in a single file, skychain notebooks create seperate page files.
These page fails are easily navigable and make it possible to maintain code in a more sustainable model.

The goal is to make a _book_ rather then a single page of content.

Here are the following tenants of the notebook:

1. The file system is the ground truth
    * Modifying the files should modify the notebook
    * The CLI provides a way to _consistently_ interact with the notebook
2. Components are standalone _viewable_, but not standalone executable
    * A page can be shared and viewed without existing in the context of a notebook
    * To execute the page, it needs to be in the context of a notebook that provides the necessary information
3. Notebooks should be editor agnostic
    * You should not require the web editor to interact, modify and execute a notebook

## Directory Structure

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
│   ├── chapter_data.iscch
│   ├── page_one.iscpg
│   └── page_two.iscpg
└── chapter_two
│   ├── chapter_data.iscch
    ├── page_one.iscpg
    └── page_two.iscpg
```

## Notebooks, Chapters, Pages, and Cells

1. Notebook
    * `iscnb` - interactive skychain notebook
    * A notebook is the top level structure.
    * It can have cells, but the intendended purpose of these cells is information / cover page info etc.
    * There may only be one `iscnb` file
    * Data
        * identifier (MEMORY ONLY)
        * directory (MEMORY ONLY)
        * chapters (MEMORY ONLY)
        * pages (MEMORY ONLY)
        * title
        * authors
            * name
            * email
            * org
            * website
        * cells
2. Chapter
    * `chapter_identifier/`
    * `chapter_identifier/<<any_name>>.iscch`
    * A directory within a notebook
    * Organize files into a more succient manner
    * There may only be one `ischh` file per subdirectory
    * Data
        * identifier (MEMORY ONLY)
        * directory (MEMORY ONLY)
        * pages (MEMORY ONLY)
        * title
        * cells
3. Page
    * `iscpg` - interactive skychain page
    * A page of code and writing
    * Data
        * identifier (MEMORY ONLY)
        * title
        * cells
4. Cell
    * A single statically typed cell of data
    * Interface
      * identifier -- unique string, most likely a UUID
      * content_hash -- hash code of the current data content
      * serialize
      * deserialize

## Kernels

## Themes

## Config

## Cache

## Notebook Directory Structure