# erltoken

Erlang token budgeting and cost estimation backed by the Rust `tiktoken` crate.

Version: `0.1.2`

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

5> erltoken:estimate_cost_usd_micro(<<"gpt-4o">>, 1000, 1000).
{ok,12500}
```

`estimate_cost_usd_micro/3` returns micro-USD. `12500` means `$0.012500`.

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
- `estimate_cost_usd_micro/3`

For token APIs, the first argument may be an encoding name such as `<<"cl100k_base">>` or a model name such as `<<"gpt-4o">>`.

## License

Apache-2.0
