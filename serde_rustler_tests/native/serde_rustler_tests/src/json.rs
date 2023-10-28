use rustler::{Env, Error as NifError, NifResult, Term};
use serde_bytes::Bytes;
use serde_rustler::{from_term, to_term, Deserializer, Serializer};
use serde_transcode::transcode;

#[inline]
/// Deserializes a JSON string into an Elixir term.
pub fn decode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let json_bytes: &[u8] = from_term(arg)?;
    let mut de = serde_json::Deserializer::from_slice(json_bytes);
    let ser = Serializer::from(env);
    transcode(&mut de, ser).map_err(|err| err.into())
}

#[inline]
#[rustler::nif]
/// Deserializes a JSON string into an Elixir term.
pub fn decode_json<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    decode(env, arg)
}

#[inline]
#[rustler::nif(schedule = "DirtyCpu")]
pub fn decode_json_dirty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    decode(env, arg)
}

#[inline]
/// Serializes an Elixir term into a compact JSON string.
pub fn encode_compact<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let de = Deserializer::from(arg);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}

#[inline]
#[rustler::nif]
pub fn encode_json_compact<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    encode_compact(env, arg)
}

#[inline]
#[rustler::nif(schedule = "DirtyCpu")]
pub fn encode_json_compact_dirty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    encode_compact(env, arg)
}

#[inline]
/// Serializes an Elixir term into a pretty-printed JSON string.
pub fn encode_pretty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let de = Deserializer::from(arg);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::pretty(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}

#[inline]
#[rustler::nif]
pub fn encode_json_pretty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    encode_pretty(env, arg)
}

#[inline]
#[rustler::nif(schedule = "DirtyCpu")]
pub fn encode_json_pretty_dirty<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    encode_pretty(env, arg)
}
