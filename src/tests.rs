use crate::{mock::*};
use frame_support::{assert_ok, assert_noop, dispatch::DispatchError};

use crate::{GameState, RawEvent};

#[test]
fn check_intial_storage() {
    ExternalityBuilder::build().execute_with(|| {
        assert_eq!(GameModule::game_state(), GameState::Ended);
    })
}


#[test]
fn start_new_game_with_root() {
    ExternalityBuilder::build().execute_with(|| {
        assert_eq!(GameModule::game_state(), GameState::Ended);

        assert_ok!(GameModule::start_new_game(Origin::root()));

        assert_eq!(
            System::events()[0].event,
            TestEvent::my_events(RawEvent::GameStarted)
        );

        assert_eq!(GameModule::game_state(), GameState::CurrentlyPlaying);
    })
}

#[test]
fn start_new_game_with_signed() {
    ExternalityBuilder::build().execute_with(|| {
        assert_eq!(GameModule::game_state(), GameState::Ended);
        assert_noop!(GameModule::start_new_game(Origin::signed(0)), DispatchError::BadOrigin);
    })
}

#[test]
fn end_current_game_with_root() {
    ExternalityBuilder::build().execute_with(|| {
        assert_ok!(GameModule::start_new_game(Origin::root()));

        assert_ok!(GameModule::end_current_game(Origin::root()));

        assert_eq!(
            System::events()[0].event,
            TestEvent::my_events(RawEvent::GameStarted)
        );

        assert_eq!(
            System::events()[1].event,
            TestEvent::my_events(RawEvent::NoWinnerInThisMatch)
        );

        assert_eq!(
            System::events()[2].event,
            TestEvent::my_events(RawEvent::GameEnded)
        );

        assert_eq!(GameModule::game_state(), GameState::Ended);
    })
}

#[test]
fn end_current_game_with_signed() {
    ExternalityBuilder::build().execute_with(|| {
        assert_ok!(GameModule::start_new_game(Origin::root()));
        assert_noop!(GameModule::end_current_game(Origin::signed(0)), DispatchError::BadOrigin);
    })
}

#[test]
fn end_current_game_with_root_after_score_update() {
    ExternalityBuilder::build().execute_with(|| {
        assert_ok!(GameModule::start_new_game(Origin::root()));

        assert_ok!(GameModule::add_score(Origin::signed(1), 20));
        assert_ok!(GameModule::add_score(Origin::signed(2), 20));
        assert_ok!(GameModule::add_score(Origin::signed(3), 40));
        assert_ok!(GameModule::add_score(Origin::signed(1), 10));

        assert_ok!(GameModule::end_current_game(Origin::root()));

        assert_eq!(
            System::events()[0].event,
            TestEvent::my_events(RawEvent::GameStarted)
        );

        assert_eq!(
            System::events()[1].event,
            TestEvent::my_events(RawEvent::NewPlayerJoined(1))
        );

        assert_eq!(
            System::events()[2].event,
            TestEvent::my_events(RawEvent::ScoreAddedFor(1, 20))
        );

        assert_eq!(
            System::events()[3].event,
            TestEvent::my_events(RawEvent::NewPlayerJoined(2))
        );

        assert_eq!(
            System::events()[4].event,
            TestEvent::my_events(RawEvent::ScoreAddedFor(2, 20))
        );

        assert_eq!(
            System::events()[5].event,
            TestEvent::my_events(RawEvent::NewPlayerJoined(3))
        );

        assert_eq!(
            System::events()[6].event,
            TestEvent::my_events(RawEvent::ScoreAddedFor(3, 40))
        );

        assert_eq!(
            System::events()[7].event,
            TestEvent::my_events(RawEvent::ScoreAddedFor(1, 30))
        );

        assert_eq!(
            System::events()[8].event,
            TestEvent::my_events(RawEvent::GameWonBy(3, 40))
        );

        assert_eq!(
            System::events()[9].event,
            TestEvent::my_events(RawEvent::GameEnded)
        );

        assert_eq!(GameModule::game_state(), GameState::Ended);

        assert_eq!(GameModule::game_records(1), 0);
        assert_eq!(GameModule::game_records(2), 0);
        assert_eq!(GameModule::game_records(3), 1);
    })
}