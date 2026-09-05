# modus-examples

Educational **dummy** plugins for **Modus ABI 2**. Not the store catalog and not copies of the closed product `plugins/` tree.

**Repos:** [modus-examples](https://github.com/CaptainGnome/modus-examples) (this repo) · [modus-sdk](https://github.com/CaptainGnome/modus-sdk) · [modus-docs](https://github.com/CaptainGnome/modus-docs)

Your own plugin always starts from `modus new`, not a fork of these dummies.

## Clone (sibling layout)

```powershell
git clone https://github.com/CaptainGnome/modus-sdk.git
git clone https://github.com/CaptainGnome/modus-docs.git
git clone https://github.com/CaptainGnome/modus-examples.git
cd modus-sdk
```

Each crate depends on `../../modus-sdk/guest` (both repos under the same parent directory).

## Layout

| Directory | Role | Try |
| --- | --- | --- |
| `consumer/` | `consumer` | `modus dev ../modus-examples/consumer` |
| `emitter/` | `emitter` | `modus dev ../modus-examples/emitter` — emits `fixture hello` + donation in `init` |
| `connector-replay/` | `connector` | `modus dev ../modus-examples/connector-replay --token fake --replay ../modus-examples/fixtures/irc.replay` |
| `widget/` | `widget` | `modus dev ../modus-examples/widget --ui` |
| `fixtures/irc.replay` | — | WS text lines for connector replay |

From the `modus-sdk` clone root (after `cargo run --manifest-path cli/Cargo.toml --release -- …` or a `modus` alias — see modus-docs):

```powershell
modus dev ../modus-examples/consumer
modus pack ../modus-examples/emitter
```

## License

MIT
