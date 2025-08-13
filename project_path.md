# Project path of explore_simple_test_case

## Restart project - delete all files and create a new project :blush:
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& mkdir tests.
```
<!-- keep the format -->
>[TIP!]
FIXME:
>Find and grep - useful for missing thinks
><!-- keep the format -->
>```bash <!-- markdownlint-disable-line code-block-style -->
>find /home/trapapa/ -name "*.md" -exec grep "EOF" {} \;
>```
<!-- -->
>[TIP!]

<!-- keep the format -->

## Create simplest test case
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->

```



<!-- keep the format -->
>[!NOTE]
>Symbol to mark web external links [![alt text][1]](./README.md)
<!-- -->
<!-- download the link sign -->
<!-- mkdir -p img && curl --create-dirs --output-dir img -O  "https://raw.githubusercontent.com/MathiasStadler/link_symbol_svg/refs/heads/main/link_symbol.svg"-->
<!-- Link sign - Don't Found a better way :-( - You know a better method? - send me a email -->
[1]: ./img/link_symbol.svg
<!-- keep the format -->
