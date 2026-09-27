# Architecture

## Storage layout

The `ticketing` contract uses instance storage and persistent storage entries:

- **Instance storage**: `Admin`, `PaymentToken`, `NextTicketId` — set once at
  `initialize` and read on nearly every call.
- **Persistent storage, keyed by `Event(u64)`**: one entry per event,
  holding the organizer address and resale policy. The issuance counter is
  kept separately in `TicketsIssued(u64)`, so minting does not rewrite the
  full event record.
- **Persistent storage, keyed by `OrganizerEvents(Address)`**: one count per
  organizer, incremented when an event is created.
- **Persistent storage, keyed by `Ticket(u64)`**: one entry per ticket,
  holding owner, tier, seat, status, and pricing.

## Why persistent storage for events and tickets

Instance storage is cheap to read but expires with the contract
instance's own TTL and isn't a good fit for data that individual
ticket owners depend on staying alive independently of contract
upgrades. Persistent storage entries are extended on every ticket write
(`extend_ttl` in `save_ticket`) so an active ticket never lapses. Event
records are extended when they are created; the full policy, the
archival behavior, and the off-chain keepalive job that covers the rest
are in [`STORAGE_TTL.md`](STORAGE_TTL.md).

## Counter placement and mint benchmark

`NextTicketId` remains in instance storage because it is one contract-wide
hot counter. Moving it to persistent storage would add a hot persistent entry
without removing the read and write required for allocation. The per-event
`TicketsIssued` counter is persistent because it is event-scoped.

The budget regression test `issue_ticket_stays_under_cost_thresholds` measures
the post-migration mint path. Comparing `env.cost_estimate().budget()` before
and after the migration demonstrates the saving from removing the full
`Event` write and keeps a ceiling against future regressions.

## Authorization model

Every state-changing function calls `require_auth()` on the account
that must have approved the action:

- `create_event`, `issue_ticket`, `check_in`, `revoke_ticket` — the
  event's organizer.
- `purchase_primary`, `buy_resale` — the buyer.
- `transfer_ticket`, `list_for_resale`, `cancel_resale` — the current
  owner.

There is no admin override for any of these — the admin set at
`initialize` is reserved for future platform-level configuration, not
per-ticket authority.

## Payment settlement

All monetary transfers go through a single SEP-41 `payment_token`
configured at `initialize`. `purchase_primary` and `buy_resale` are
the only functions that move funds; both do so atomically with the
ownership change in the same transaction.
