-module(erltoken).

-on_load(init/0).

-export([
    list_encodings/0,
    encoding_for_model/1,
    count/2,
    encode/2,
    encode_with_special_tokens/2,
    decode/2,
    fits/3,
    trim_to_token_limit/3,
    remaining/3,
    estimate_cost_usd_micro/3
]).

-export([
    list_encodings_nif/0,
    encoding_for_model_nif/1,
    count_nif/2,
    encode_nif/2,
    encode_with_special_tokens_nif/2,
    decode_nif/2,
    estimate_cost_usd_micro_nif/3
]).

-type name() :: binary().
-type token() :: non_neg_integer().

-spec init() -> ok | {error, term()}.
init() ->
    SoName = "erltoken_nif",
    case code:priv_dir(erltoken) of
        PrivDir when is_list(PrivDir) ->
            erlang:load_nif(filename:join(PrivDir, SoName), 0);
        {error, bad_name} ->
            erlang:load_nif(filename:join("priv", SoName), 0)
    end.

-spec list_encodings() -> [binary()].
list_encodings() ->
    list_encodings_nif().

-spec encoding_for_model(binary()) -> {ok, binary()} | {error, unknown_model | invalid_utf8}.
encoding_for_model(Model) when is_binary(Model) ->
    encoding_for_model_nif(Model).

-spec count(name(), binary()) -> {ok, non_neg_integer()} | {error, term()}.
count(Name, Text) when is_binary(Name), is_binary(Text) ->
    count_nif(Name, Text).

-spec encode(name(), binary()) -> {ok, [token()]} | {error, term()}.
encode(Name, Text) when is_binary(Name), is_binary(Text) ->
    encode_nif(Name, Text).

-spec encode_with_special_tokens(name(), binary()) -> {ok, [token()]} | {error, term()}.
encode_with_special_tokens(Name, Text) when is_binary(Name), is_binary(Text) ->
    encode_with_special_tokens_nif(Name, Text).

-spec decode(name(), [token()]) -> {ok, binary()} | {error, term()}.
decode(Name, Tokens) when is_binary(Name), is_list(Tokens) ->
    decode_nif(Name, Tokens).

-spec fits(name(), binary(), non_neg_integer()) -> {ok, boolean()} | {error, term()}.
fits(Name, Text, MaxTokens) when is_binary(Name), is_binary(Text), is_integer(MaxTokens), MaxTokens >= 0 ->
    case count(Name, Text) of
        {ok, Count} -> {ok, Count =< MaxTokens};
        {error, _} = Error -> Error
    end.

-spec trim_to_token_limit(name(), binary(), non_neg_integer()) -> {ok, binary()} | {error, term()}.
trim_to_token_limit(Name, Text, MaxTokens) when is_binary(Name), is_binary(Text), is_integer(MaxTokens), MaxTokens >= 0 ->
    case encode(Name, Text) of
        {ok, Tokens} when length(Tokens) =< MaxTokens ->
            {ok, Text};
        {ok, Tokens} ->
            decode(Name, lists:sublist(Tokens, MaxTokens));
        {error, _} = Error ->
            Error
    end.

-spec remaining(name(), binary(), non_neg_integer()) -> {ok, non_neg_integer()} | {error, term()}.
remaining(Name, Text, ContextWindow) when is_binary(Name), is_binary(Text), is_integer(ContextWindow), ContextWindow >= 0 ->
    case count(Name, Text) of
        {ok, Count} when Count =< ContextWindow -> {ok, ContextWindow - Count};
        {ok, _Count} -> {ok, 0};
        {error, _} = Error -> Error
    end.

-spec estimate_cost_usd_micro(binary(), non_neg_integer(), non_neg_integer()) ->
    {ok, non_neg_integer()} | {error, term()}.
estimate_cost_usd_micro(Model, InputTokens, OutputTokens)
    when is_binary(Model), is_integer(InputTokens), InputTokens >= 0,
         is_integer(OutputTokens), OutputTokens >= 0 ->
    estimate_cost_usd_micro_nif(Model, InputTokens, OutputTokens).

list_encodings_nif() ->
    erlang:nif_error(nif_not_loaded).

encoding_for_model_nif(_Model) ->
    erlang:nif_error(nif_not_loaded).

count_nif(_Name, _Text) ->
    erlang:nif_error(nif_not_loaded).

encode_nif(_Name, _Text) ->
    erlang:nif_error(nif_not_loaded).

encode_with_special_tokens_nif(_Name, _Text) ->
    erlang:nif_error(nif_not_loaded).

decode_nif(_Name, _Tokens) ->
    erlang:nif_error(nif_not_loaded).

estimate_cost_usd_micro_nif(_Model, _InputTokens, _OutputTokens) ->
    erlang:nif_error(nif_not_loaded).
