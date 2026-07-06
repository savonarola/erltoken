# erltoken

[![Hex.pm version](https://img.shields.io/hexpm/v/erltoken.svg)](https://hex.pm/packages/erltoken)
[![CI](https://github.com/savonarola/erltoken/actions/workflows/ci.yml/badge.svg)](https://github.com/savonarola/erltoken/actions/workflows/ci.yml)
![No Claude](https://img.shields.io/badge/%F0%9F%9A%AB_no-claude-green)

Erlang token budgeting backed by the Rust [`tiktoken-rs`](https://docs.rs/tiktoken-rs) crate.

Version: `0.2.0`

All string inputs and outputs are binaries.

## Versioning

```sh
bump2version patch
bump2version minor
bump2version major
```

Version bumps update the README, Erlang `.app.src`, and Rust package metadata together.

## Build

```sh
rebar3 compile
```

## Examples

```erlang
1> erltoken:encoding_for_model(<<"gpt-4o">>).
{ok,<<"o200k_base">>}

2> erltoken:count(<<"gpt-4o">>, <<"hello world">>).
{ok,2}

3> erltoken:fits(<<"gpt-4o">>, <<"hello world">>, 10).
{ok,true}

4> erltoken:trim_to_token_limit(<<"cl100k_base">>, <<"hello world">>, 1).
{ok,<<"hello">>}
```

## API

- `list_encodings/0`
- `encoding_for_model/1`
- `count/2`
- `encode/2`
- `encode_with_special_tokens/2`
- `decode/2`
- `fits/3`
- `remaining/3`
- `trim_to_token_limit/3`

For token APIs, the first argument may be an encoding name such as `<<"cl100k_base">>` or a model name such as `<<"gpt-4o">>`.

## License

Apache-2.0
