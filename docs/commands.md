# Skychain Commands

* `<<no command>>` - Start the notebook server
* `init` - Create a new notebook in the current directory
* `add`
  * `kernel <kernel_identifier>` - add a kernel to the notebook
    * `<kernel_identifier> = bash` - add the first-party kernel for bash commands
    * `<kernel_identifier> = python` - add the first-party kernel for python code
    * `<kernel_identifier> = <git directory>` - download a third-party kernel from github
  * `theme <theme_identifier>`
    * `<theme_identifier> = <none>` - generate a default theme file
    * `<theme_identifier> = light` - add the first-party light mode theme
    * `<theme_identifier> = dark` - add the first-party dark mode theme
  * `chapter <chapter_title>`
  * `page <page_title>`
* `config`