# Pretty printing

A simple implementation based on [Justin's blog](https://justinpombrio.net/2024/02/23/a-twist-on-Wadlers-printer.html)
It's tailored for [afmt repo](https://github.com/xixiaofinland/afmt).

## Use Cases

There are a couple of json files in `examples` folder.
For example, you can run the command below to see the different printing result.

```bash
# the last number is the line max_width.
cargo run --release --example json examples/4.json 20
cargo run --release --example json examples/4.json 37
cargo run --release --example json examples/4.json 40
cargo run --release --example json examples/4.json 120
```
