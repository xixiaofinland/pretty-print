# Pretty printing

A simple implementation based on [Justin's blog](https://justinpombrio.net/2024/02/23/a-twist-on-Wadlers-printer.html)
It's tailored for [afmt repo](https://github.com/xixiaofinland/afmt).

## Use Cases

There are a couple of json files in `examples` folder. You can run the command below to check the
output.


```bash
cargo run --release --example json examples/4.json 40 # the last number is the printing line max_width.
```
