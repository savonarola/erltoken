use rustler::{atoms, Binary, Encoder, Env, NewBinary, Term};
use tiktoken_rs::{tokenizer::Tokenizer, CoreBPE};

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

fn tokenizer_name(tokenizer: Tokenizer) -> &'static str {
    match tokenizer {
        Tokenizer::O200kHarmony => "o200k_harmony",
        Tokenizer::O200kBase => "o200k_base",
        Tokenizer::Cl100kBase => "cl100k_base",
        Tokenizer::P50kBase => "p50k_base",
        Tokenizer::R50kBase => "r50k_base",
        Tokenizer::P50kEdit => "p50k_edit",
        Tokenizer::Gpt2 => "gpt2",
    }
}

fn tokenizer_for_name(name: &str) -> Option<Tokenizer> {
    match name {
        "o200k_harmony" => Some(Tokenizer::O200kHarmony),
        "o200k_base" => Some(Tokenizer::O200kBase),
        "cl100k_base" => Some(Tokenizer::Cl100kBase),
        "p50k_base" => Some(Tokenizer::P50kBase),
        "r50k_base" => Some(Tokenizer::R50kBase),
        "p50k_edit" => Some(Tokenizer::P50kEdit),
        "gpt2" => Some(Tokenizer::Gpt2),
        _ => tiktoken_rs::tokenizer::get_tokenizer(name),
    }
}

fn resolve_encoding(name: &str) -> Option<&'static CoreBPE> {
    tokenizer_for_name(name).and_then(|tokenizer| tiktoken_rs::bpe_for_tokenizer(tokenizer).ok())
}

#[rustler::nif]
fn list_encodings_nif<'a>(env: Env<'a>) -> Term<'a> {
    let items: Vec<Term<'a>> = [
        "o200k_harmony",
        "o200k_base",
        "cl100k_base",
        "p50k_base",
        "r50k_base",
        "p50k_edit",
        "gpt2",
    ]
    .into_iter()
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

    match tiktoken_rs::tokenizer::get_tokenizer(model) {
        Some(tokenizer) => (ok(), binary_str_term(env, tokenizer_name(tokenizer))).encode(env),
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
        Some(encoding) => (ok(), encoding.count_ordinary(text)).encode(env),
        None => (error(), unknown_encoding_or_model()).encode(env),
    }
}

#[rustler::nif]
fn encode_nif<'a>(env: Env<'a>, name: Binary<'a>, text: Binary<'a>) -> Term<'a> {
    encode_impl(env, name, text, false)
}

#[rustler::nif]
fn encode_with_special_tokens_nif<'a>(
    env: Env<'a>,
    name: Binary<'a>,
    text: Binary<'a>,
) -> Term<'a> {
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
                encoding.encode_ordinary(text)
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
        Some(encoding) => match encoding.decode_bytes(&tokens) {
            Ok(bytes) => (ok(), binary_term(env, &bytes)).encode(env),
            Err(_) => (error(), unknown_encoding_or_model()).encode(env),
        },
        None => (error(), unknown_encoding_or_model()).encode(env),
    }
}

rustler::init!("erltoken");
