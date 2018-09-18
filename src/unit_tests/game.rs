use game::Game;

#[test]
fn game_score_nothing_is_valid() {
    // given a game validate function
    let rolls = &[];
    let expected_result = Result::Ok([].as_ref());
    let validate = Game::validate;

    // when an empty slice is validated
    let result = validate(rolls);

    // then the result should be OK
    assert_eq!(result, expected_result);
}

#[test]
fn game_score_nothing_yields_none() {
    // given a game score function
    let rolls = &[];
    let expected_result = Option::None::<u16>;
    let score = Game::score;

    // when an empty slice is provided
    let result = score(rolls);

    // then the result should be None
    assert_eq!(result, expected_result);
}
