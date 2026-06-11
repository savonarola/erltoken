use rustler::{atoms, Binary, Encoder, Env, NewBinary, Term};
use tiktoken::CoreBpe;

atoms! {
    ok,
    error,
    invalid_utf8,
    unknown_encoding_or_model,
    unknown_model
}

fn binary_term<'a>(env: Env<'a>, bytes: &[u8]) -> Term<'a> {
    let mut bin = NewBinary::new(env, bytes.len());
    bin.as_mut_slice().copy_from_slice(bytes);
    bin.into()
}

fn binary_str_term<'a>(env: Env<'a>, value: &str) -> Term<'a> {
    binary_term(env, value.as_bytes())
}

fn binary_to_str<'a>(env: Env<'a>, bin: Binary<'a>) -> Result<&'a str, Term<'a>> {
    std::str::from_utf8(bin.as_slice()).map_err(|_| invalid_utf8().encode(env))
}

fn resolve_encoding(name: &str) -> Option<&'static CoreBpe> {
    tiktoken::get_encoding(name)
        .or_else(|| tiktoken::model_to_encoding(name).and_then(tiktoken::get_encoding))
}

#[rustler::nif]
fn list_encodings_nif<'a>(env: Env<'a>) -> Term<'a> {
    let items: Vec<Term<'a>> = tiktoken::list_encodings()
        .iter()
        .map(|name| binary_str_term(env, name))
        .collect();
    items.encode(env)
}

#[rustler::nif]
fn encoding_for_model_nif<'a>(env: Env<'a>, model: Binary<'a>) -> Term<'a> {
    let model = match binary_to_str(env, model) {
        Ok(model) => model,
        Err(reason) => return (error(), reason).encode(env),
    };

    match tiktoken::model_to_encoding(model) {
        Some(name) => (ok(), binary_str_term(env, name)).encode(env),
        None => (error(), unknown_model()).encode(env),
    }
}

#[rustler::nif]
fn count_nif<'a>(env: Env<'a>, name: Binary<'a>, text: Binary<'a>) -> Term<'a> {
    let name = match binary_to_str(env, name) {
        Ok(name) => name,
        Err(reason) => return (error(), reason).encode(env),
    };
    let text = match binary_to_str(env, text) {
        Ok(text) => text,
        Err(reason) => return (error(), reason).encode(env),
    };

    match resolve_encoding(name) {
        Some(encoding) => (ok(), encoding.count(text)).encode(env),
        None => (error(), unknown_encoding_or_model()).encode(env),
    }
}

#[rustler::nif]
fn encode_nif<'a>(env: Env<'a>, name: Binary<'a>, text: Binary<'a>) -> Term<'a> {
    encode_impl(env, name, text, false)
}

#[rustler::nif]
fn encode_with_special_tokens_nif<'a>(env: Env<'a>, name: Binary<'a>, text: Binary<'a>) -> Term<'a> {
    encode_impl(env, name, text, true)
}

fn encode_impl<'a>(env: Env<'a>, name: Binary<'a>, text: Binary<'a>, special: bool) -> Term<'a> {
    let name = match binary_to_str(env, name) {
        Ok(name) => name,
        Err(reason) => return (error(), reason).encode(env),
    };
    let text = match binary_to_str(env, text) {
        Ok(text) => text,
        Err(reason) => return (error(), reason).encode(env),
    };

    match resolve_encoding(name) {
        Some(encoding) => {
            let tokens = if special {
                encoding.encode_with_special_tokens(text)
            } else {
                encoding.encode(text)
            };
            (ok(), tokens).encode(env)
        }
        None => (error(), unknown_encoding_or_model()).encode(env),
    }
}

#[rustler::nif]
fn decode_nif<'a>(env: Env<'a>, name: Binary<'a>, tokens: Vec<u32>) -> Term<'a> {
    let name = match binary_to_str(env, name) {
        Ok(name) => name,
        Err(reason) => return (error(), reason).encode(env),
    };

    match resolve_encoding(name) {
        Some(encoding) => (ok(), binary_term(env, &encoding.decode(&tokens))).encode(env),
        None => (error(), unknown_encoding_or_model()).encode(env),
    }
}

#[rustler::nif]
fn estimate_cost_usd_micro_nif<'a>(
    env: Env<'a>,
    model: Binary<'a>,
    input_tokens: u64,
    output_tokens: u64,
) -> Term<'a> {
    let model = match binary_to_str(env, model) {
        Ok(model) => model,
        Err(reason) => return (error(), reason).encode(env),
    };

    match tiktoken::pricing::estimate_cost(model, input_tokens, output_tokens) {
        Some(cost) => {
            let micros = (cost * 1_000_000.0).round() as u64;
            (ok(), micros).encode(env)
        }
        None => (error(), unknown_model()).encode(env),
    }
}

rustler::init!("erltoken");
