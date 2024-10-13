use std::collections::HashSet;

use ndarray::prelude::*;
use crate::game::{Card, CARD_LIST, RoundState};

fn card_to_multi_hot(card_list: &[Card]) -> [f32; 48] {
    let mut card_multi_hot = [0f32; 48];
    for (x, y) in card_list {
        card_multi_hot[((x-1)*4+(y-1)) as usize] = 1f32;
    }
    card_multi_hot
}

fn card_list_to_set(card_list: Vec<Card>) -> HashSet<Card> {
    card_list.into_iter().collect()
}

fn reserve_array() -> Array2<f32> {
    Array::zeros((17, 48))
}

fn inter_len(slice: &Vec<Card>, set: &HashSet<Card>) -> usize {
    slice.iter().filter(|card| set.contains(card)).count()
}

fn yaku_status_array(state: &RoundState) -> Array2<f32> {
    let turn_player = state.turn_player();
    let idle_player = 1 - turn_player;

    let my_hand_cards: HashSet<_> = state.hand[turn_player].iter().copied().collect();
    let board_cards: HashSet<_> = state.field().iter().copied().collect();
    let my_collect_cards: HashSet<_> = state.pile[turn_player].iter().copied().collect();
    let op_collect_cards: HashSet<_> = state.pile[idle_player].iter().copied().collect();
    let mut unseen_cards: HashSet<_> = state.hand[idle_player].iter().copied().collect();
    for card in &state.stock {
        unseen_cards.insert(*card);
    };
    let mut card_state = vec!();
    for cards in CARD_LIST.iter() {
        card_state.push(inter_len(cards, &my_hand_cards) as f32);
    }
    for cards in CARD_LIST.iter() {
        card_state.push(inter_len(cards, &board_cards) as f32);
    }
    for cards in CARD_LIST.iter() {
        card_state.push(inter_len(cards, &my_collect_cards) as f32);
    }
    for cards in CARD_LIST.iter() {
        card_state.push(inter_len(cards, &op_collect_cards) as f32);
    }
    for cards in CARD_LIST.iter() {
        card_state.push(inter_len(cards, &unseen_cards) as f32);
    }

    let card_state: Array2<f32> = Array2::from_shape_vec((3, 1), card_state).unwrap();
    let card_state: ArrayView2<f32> = card_state.broadcast((card_state.nrows(), 48)).unwrap();
    let card_state: Array2<f32>  = card_state.t().to_owned();

    let mut card_key: Vec<_> = Vec::new();
    for cards in  CARD_LIST.iter() {
        for v in card_to_multi_hot(cards) {
            card_key.push(v);
        }
    }
    let card_key = Array2::from_shape_vec((CARD_LIST.len(), 48), card_key).unwrap();
    ndarray::concatenate!(Axis(0), card_state, card_key)
}

fn card_suit_array() -> Array2<f32> {
    let mut array = Array::zeros((12,48));
    for i in 0..12 {
        array.slice_mut(s![i, 4*i..4*i+4]).fill(1.0);
    }
    array
}

fn card_init_position_array(state: &RoundState) -> Array2<f32> {
    let turn_player = state.turn_player();
    let cards_nn_my_hand = card_to_multi_hot(&state.hand[turn_player]);
    //let cards_in_board = card_to_multi_hot(self.log['basic']['initBoard'])
    let unseen_cards = card_to_multi_hot(&state.unseen_cards(turn_player));
    // todo
    ndarray::stack!(Axis(0), cards_nn_my_hand, unseen_cards)
}