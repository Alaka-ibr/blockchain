//! Event label validation for `create_event` (Issue #125).
//!
//! `create_event` accepted empty `name` and `category` strings, so an event
//! could be created with no human-readable identity. Both are now rejected
//! when empty, along with the oversized case handled in
//! [`crate::test::ticket_labels`].

use super::*;

#[test]
fn create_event_rejects_empty_name() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let result = client.try_create_event(
        &organizer,
        &1,
        &String::from_str(&env, ""),
        &String::from_str(&env, "concert"),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    assert_eq!(result, Err(Ok(Error::EmptyNameOrCategory)));
}

#[test]
fn create_event_rejects_empty_category() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let result = client.try_create_event(
        &organizer,
        &1,
        &String::from_str(&env, "Radiohead Live"),
        &String::from_str(&env, ""),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    assert_eq!(result, Err(Ok(Error::EmptyNameOrCategory)));
}

#[test]
fn create_event_rejects_whitespace_free_single_char_boundary() {
    // One byte is the smallest accepted label, MAX_* is the largest.
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    client.create_event(
        &organizer,
        &1,
        &String::from_str(&env, "A"),
        &String::from_str(&env, "c"),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    let event = client.get_event(&1);
    assert_eq!(event.name.len(), 1);
    assert_eq!(event.category.len(), 1);
}

#[test]
fn create_event_accepts_labels_at_the_maximum_length() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let name = "n".repeat(MAX_NAME_LEN as usize);
    let category = "c".repeat(MAX_CATEGORY_LEN as usize);

    client.create_event(
        &organizer,
        &1,
        &String::from_str(&env, &name),
        &String::from_str(&env, &category),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    let event = client.get_event(&1);
    assert_eq!(event.name.len(), MAX_NAME_LEN);
    assert_eq!(event.category.len(), MAX_CATEGORY_LEN);
}

#[test]
fn create_event_rejects_oversized_name() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let name = "n".repeat(MAX_NAME_LEN as usize + 1);
    let result = client.try_create_event(
        &organizer,
        &1,
        &String::from_str(&env, &name),
        &String::from_str(&env, "concert"),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    assert_eq!(result, Err(Ok(Error::StringTooLong)));
}

#[test]
fn create_event_rejects_oversized_category() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let category = "c".repeat(MAX_CATEGORY_LEN as usize + 1);
    let result = client.try_create_event(
        &organizer,
        &1,
        &String::from_str(&env, "Radiohead Live"),
        &String::from_str(&env, &category),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );
    assert_eq!(result, Err(Ok(Error::StringTooLong)));
}

#[test]
fn create_event_with_options_rejects_empty_name() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let result = client.try_create_event_with_options(
        &organizer,
        &1,
        &String::from_str(&env, ""),
        &String::from_str(&env, "concert"),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
        &None,
        &None,
    );
    assert_eq!(result, Err(Ok(Error::EmptyNameOrCategory)));
}

#[test]
fn rejected_event_is_not_persisted() {
    let (env, client, _token, _token_asset, _admin, organizer) = setup();

    let _ = client.try_create_event(
        &organizer,
        &7,
        &String::from_str(&env, ""),
        &String::from_str(&env, "concert"),
        &12_000u32,
        &500u32,
        &10_000u64,
        &100u64,
        &200u64,
    );

    assert_eq!(client.try_get_event(&7), Err(Ok(Error::EventNotFound)));
}
