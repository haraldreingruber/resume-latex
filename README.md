# Resume LaTeX Project

This workspace contains a LaTeX resume template tailored for a senior software engineer, built with [AltaCV](https://github.com/liantze/AltaCV) (two-column, sidebar-style layout).

## Getting started

1. Install MiKTeX from https://miktex.org/download
2. Install the VS Code extension LaTeX Workshop
3. Open this folder in VS Code
4. Build the project with the command `LaTeX Workshop: Build LaTeX project` (compiles with `pdflatex`)

The compiled PDF will be generated as `main.pdf`.

## Customization

Edit `main.tex` to replace the placeholder content with your own experience, achievements, and contact details. `altacv.cls` is the template's class file, vendored locally since it isn't distributed as a MiKTeX/CTAN package.

## Other templates

The `examples/` folder has alternate resume templates with the same content, for comparison:

- `examples/altacv/` — a standalone copy of the AltaCV template now used at the project root
- `examples/moderncv/` — the original single-column template (moderncv, banking style), builds with `pdflatex`
- `examples/awesome-cv/` — a colored single-column template, builds with `xelatex` (fonts substituted with Calibri/Segoe UI; see comments in `awesome-cv.cls`)
