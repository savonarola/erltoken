-module(erltoken_tests).

-include_lib("eunit/include/eunit.hrl").

list_encodings_test() ->
    Names = erltoken:list_encodings(),
    ?assert(lists:member(<<"cl100k_base">>, Names)),
    ?assert(lists:member(<<"o200k_base">>, Names)).

encoding_for_model_test() ->
    ?assertEqual({ok, <<"o200k_base">>}, erltoken:encoding_for_model(<<"gpt-4o">>)),
    ?assertEqual({error, unknown_model}, erltoken:encoding_for_model(<<"not-a-model">>)).

count_and_encode_by_encoding_test() ->
    Text = <<"hello world">>,
    ?assertEqual({ok, 2}, erltoken:count(<<"cl100k_base">>, Text)),
    ?assertEqual({ok, [15339, 1917]}, erltoken:encode(<<"cl100k_base">>, Text)).

count_accepts_model_name_test() ->
    ?assertEqual({ok, 2}, erltoken:count(<<"gpt-4o">>, <<"hello world">>)).

decode_test() ->
    ?assertEqual({ok, <<"hello world">>}, erltoken:decode(<<"cl100k_base">>, [15339, 1917])).

special_tokens_test() ->
    Text = <<"hello<|endoftext|>world">>,
    {ok, Ordinary} = erltoken:encode(<<"cl100k_base">>, Text),
    {ok, Special} = erltoken:encode_with_special_tokens(<<"cl100k_base">>, Text),
    ?assertNot(lists:member(100257, Ordinary)),
    ?assert(lists:member(100257, Special)).

fits_and_remaining_test() ->
    Text = <<"hello world">>,
    ?assertEqual({ok, true}, erltoken:fits(<<"cl100k_base">>, Text, 2)),
    ?assertEqual({ok, false}, erltoken:fits(<<"cl100k_base">>, Text, 1)),
    ?assertEqual({ok, 8}, erltoken:remaining(<<"cl100k_base">>, Text, 10)),
    ?assertEqual({ok, 0}, erltoken:remaining(<<"cl100k_base">>, Text, 1)).

trim_to_token_limit_test() ->
    Text = <<"hello world">>,
    ?assertEqual({ok, Text}, erltoken:trim_to_token_limit(<<"cl100k_base">>, Text, 2)),
    ?assertEqual({ok, <<"hello">>}, erltoken:trim_to_token_limit(<<"cl100k_base">>, Text, 1)),
    ?assertEqual({ok, <<>>}, erltoken:trim_to_token_limit(<<"cl100k_base">>, Text, 0)).

estimate_cost_usd_micro_test() ->
    ?assertEqual({ok, 12500}, erltoken:estimate_cost_usd_micro(<<"gpt-4o">>, 1000, 1000)),
    ?assertEqual({error, unknown_model}, erltoken:estimate_cost_usd_micro(<<"not-a-model">>, 1000, 1000)).

unknown_encoding_test() ->
    ?assertEqual({error, unknown_encoding_or_model}, erltoken:count(<<"not-an-encoding">>, <<"hello">>)).
