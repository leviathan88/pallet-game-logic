#![cfg_attr(not(feature = "std"), no_std)]

use codec::{ Decode, Encode };
use frame_support::{
	decl_module, decl_storage, decl_event, 
	decl_error, dispatch::DispatchResult,
	ensure,
};
use sp_std::prelude::Vec;
use frame_system::{ensure_root, ensure_signed};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as GamePallet {
		CurrentGameState get(fn game_state): GameState=GameState::Ended;
		CurrentResults get(fn result_pool): map hasher(blake2_128_concat) T::AccountId => u64;
		PlayersPool get(fn players): Vec<T::AccountId>;
		GameRecords get(fn game_records): map hasher(blake2_128_concat) T::AccountId => u64;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		GameStarted,
		GameEnded,
		GameWonBy(AccountId, u64),
		NewPlayerJoined(AccountId),
		ScoreAddedFor(AccountId, u64),
		NoWinnerInThisMatch,
	}
);

decl_error! {
	pub enum Error for Module<T: Trait> {
		GameAlreadyStarted,
		GameAlreadyEnded,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		
		fn deposit_event() = default;

		#[weight = 10_000]
		fn start_new_game(origin) -> DispatchResult {
			// Only root can start the game
			let _ = ensure_root(origin)?;

			// check has the game already started
			ensure!(Self::game_state() == GameState::Ended, Error::<T>::GameAlreadyStarted);

			// start the new game
			<CurrentGameState>::put(GameState::CurrentlyPlaying);
			Self::deposit_event(RawEvent::GameStarted);

			Ok(())
		}

		#[weight = 10_000]
		fn end_current_game(origin) -> DispatchResult {
			// Only root can end the game
			let _ = ensure_root(origin)?;

			// check has the game already ended
			ensure!(Self::game_state() == GameState::CurrentlyPlaying, Error::<T>::GameAlreadyEnded);

			// choose a winner
			match Self::get_winner() {
				Some((winner, result)) => {
					// output the winner
					Self::deposit_event(RawEvent::GameWonBy(winner, result));
				},
				None => {
					Self::deposit_event(RawEvent::NoWinnerInThisMatch);
				}
			}

			// end the game
			<CurrentGameState>::put(GameState::Ended);
			Self::deposit_event(RawEvent::GameEnded);

			Ok(())
		}

		#[weight = 10_000]
		fn add_score(origin, new_score: u64) -> DispatchResult {
			// player can add score only for himself
			let player = ensure_signed(origin)?;

			// check has the game already started
			ensure!(Self::game_state() == GameState::CurrentlyPlaying, Error::<T>::GameAlreadyEnded);

			// check is new player
			if !<GameRecords<T>>::contains_key(&player) {
				Self::deposit_event(RawEvent::NewPlayerJoined(player.clone()));
				<GameRecords<T>>::insert(&player, 0);
				<CurrentResults<T>>::insert(&player, new_score);

				// add to player pool
				let mut players = <PlayersPool<T>>::get();
				players.push(player.clone());
				<PlayersPool<T>>::put(players);

				Self::deposit_event(RawEvent::ScoreAddedFor(player, new_score));
			} else {
				// check is player alredy in the current match
				if <CurrentResults<T>>::contains_key(&player) {
					let current_score = <CurrentResults<T>>::get(&player);
					let updated_score = current_score + new_score;
					<CurrentResults<T>>::insert(&player, updated_score);
					Self::deposit_event(RawEvent::ScoreAddedFor(player, updated_score));
				} else {
					<CurrentResults<T>>::insert(&player, new_score);
					Self::deposit_event(RawEvent::ScoreAddedFor(player, new_score));
				}
			}

			Ok(())
		}
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum GameState {
	CurrentlyPlaying,
	Ended,
}

impl Default for GameState {
	fn default() -> Self { Self::Ended }
}

impl<T: Trait> Module<T> {
	fn get_winner() -> Option<(T::AccountId, u64)> {
		let players = PlayersPool::<T>::get();		

		let mut best_player: Option<(T::AccountId, u64)> = None;

		for player in players {
			if <CurrentResults<T>>::contains_key(&player) {
				let current_score = <CurrentResults<T>>::get(&player);

				if let Some(ref temp) = best_player {
					if current_score > temp.1 {
						best_player = Some((player.clone(), current_score));
					}
				} else {
					best_player = Some((player.clone(), current_score));
				}

				<CurrentResults<T>>::remove(&player);
			}
		}

		Self::update_game_records(&best_player);
		best_player
	}

	fn update_game_records(player: &Option<(T::AccountId, u64)>) {
		if let Some((player_address, _)) = player {
			let current_score = <GameRecords<T>>::get(&player_address);
			<GameRecords<T>>::insert(&player_address, current_score + 1);
		}
	}
}