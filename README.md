# SkyChain Notebook

A [literate programming](http://literateprogramming.com/) tool that makes it easy to interact, edit, visualize and, share code.

## Development

```bash
git clone ...
cd skychain-notebook
cargo run
```

### Etiquette

1. Use [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/)
    * commit prefix types:
      * `feat` - see standard
      * `fix` - see standard
      * `docs` - modifying the documents in some way
      * `chore` - modifying something trivial - spelling mistake, version bumps etc.

### Roadmap

#### 0.1.x - Notebook structure and data

1. Notebook
    * [x] Initialize the notebook
    * [x] Load the notebook from the current directory
    * [x] Save the notebook into the current directory
2. Add new components
    * [ ] Add a new chapter to the notebook
    * [ ] Add a new page to the notebook if in root directory
    * [ ] Add a new page to the chapter if in chapter directory
3. Add plugin-style cells
    * [ ] Cell API
    * [ ] Markdown Cell
    * [ ] (OPTIONAL) HTML Cell
    * [ ] (OPTIONAL) Txt Cell
4. Config
    * Global
        * [ ] create global config if doesn't exist
        * [ ] modify global config option
        * [ ] open global config in default text editor
    * Local
        * [ ] create local config from global config if exists
        * [ ] modify local config option
5. Command Line Interface

#### 0.2.x - Web Viewer

1. Viewing
2. Editing
3. Themes
    * Default dark
    * Default light
    * Theme switcher
    * Custom themes

#### 0.3.x - Kernels

#### 0.4.x - Dependency & Execution chains

#### 0.5.x - Export

1. Freeze notebook
2. Export to HTML
3. Export to PDF?

#### 0.6.x - Jupyter Notebook Conversion

1. Convert a jupyter notebook into a skychain notebook

## Information

### Motivation

You can read the [manifesto](docs/motivation/manifesto.md) if you like, but, ultimately, the reasons are '_[rewrite it in rust](https://github.com/casey/awesome-rewrite-it-in-rust)_' and '_why not?_'
