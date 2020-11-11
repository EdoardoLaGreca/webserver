# webserver
![GitHub repo size](https://img.shields.io/github/repo-size/EdoardoLaGreca/webserver)
![GitHub last commit](https://img.shields.io/github/last-commit/EdoardoLaGreca/webserver)
![Lines of code](https://img.shields.io/tokei/lines/github/EdoardoLaGreca/webserver)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/EdoardoLaGreca/webserver/Rust)  
This webserver is used to provide content on [elagreca.dev](https://elagreca.dev)

## Compile & run
The only dependency is Cargo, which is the Rust Language package manager. If you're on Linux or a Unix-like OS (such as \*BSD) you should be able to install it using your OS package manager. If you can't or you're on Windows, use [this](https://rustup.rs).  

To generate the final executable (optimized), use the command below.
```
cargo build --release --target-dir bin
```
By running it, you generate an executable in the `bin/` directory which you can run through this command.
```
./bin/webserver
```

## Usage
Files and directories:
  - `src/` = Contains the webserver source code. You can ignore this if you just want to use the webserver. Otherwise, check [here](src/).
  - `www/` = Contains the files that can be accessed through the HTTP requests.
    - `www/index.md` = Is the root/initial page (`/`).
    - `www/icon/` = Contains your icons such as `favicon.ico`.
    - `www/[lang]/` where `[lang]` is a language such as `it` or `en` = Contains the real website pages, for instance: `www/en/my-page.md`.
    - `www/style/` = Contains everything related to the website style, such as stylesheets, fonts, etc...
    - `www/style/font` = Contains the fonts used in the website. By convention (**MY** convention, if it doesn't already exist), every font family has its own directory (e.g. `www/style/font/RobotoMono/RobotoMono-Regular.ttf`)
    - `www/meta.json` = Is the file where you can write metadata to enhance the website. More info below.

## `meta.json`
As already stated, this file can be used to make some enhancements to the website. Below there is an example.
``` JSON
{
    "pages": [
        {
            "filename": "my-page.md",
            "title": "My page title",
            "lang": "en",
            "path": "/en/my-page.md",
            "translations": [
                {
                    "lang": "it",
                    "path": "/it/la-mia-pagina.md"
                }
            ],
            "styles": [
                "my_stylesheet.css",
                "another_stylesheet.scss"
            ]
        }
    ]
}
```
Let's analyze it line-by-line:  

``` JSON
"pages": [

]
```
This is where the page settings are, don't change `"pages"` into something else since the webserver wouldn't recognize it anymore.

``` JSON
"filename": "my-page.md"
```
This is the name of the file which we want to enhance.

``` JSON
"title": "My page title"
```
This is the page title. When the webserver generates the HTML document from the Markdown file, this string is put into the `<title>` tag. In this case it would be something like this: `<title> My page title </title>`. If no title is set (empty string or variable doesn't exist), the webserver will try to make one based on the file name, by leaving only alphanumeric characters and removing the others.

``` JSON
"lang": "en"
```
This represents the current page language, it will be put at the beginning of the HTML document. E.g. `<html lang="en"> ... </html>`.

``` JSON
"path": "/en/my-page.md"
```
This is the path where the file (`"filename": "..."`) is located into. The path root is always `www/`, so in this case the real path is `www/en/my-page.md`.

``` JSON
"translations": [

]
```
In some cases, you may want to translate the pages in another language. This array contains the locations of the same file content in other languages. Once the page will be displayed, the user who requested the page will be able to easily switch to another language that is more comfortable for him.

``` JSON
{
    "lang": "it",
    "path": "/it/la-mia-pagina.md"
}
```
In this case, the page has been translated in Italian and the translated file is located in `www/it/la-mia-pagina.md`.

``` JSON
"styles": [
    "my_stylesheet.css",
    "another_stylesheet.scss"
]
```
In the `"styles"` array you can list the stylesheets that must be included with the markdown file. Note that this webserver supports Sass compilation and if a Sass file gets requested, it will be compiled into a CSS file in real time (inside RAM) and sent.


## License
This repository uses a BSD-3-Clause license. More info [here](https://github.com/EdoardoLaGreca/webserver/blob/main/LICENSE).
