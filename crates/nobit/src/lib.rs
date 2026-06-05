/* TODO:
 * - id() -> id
 * - add(id, concept, nature, quantity) -> id | created_at
 * - get(id) -> Entry
 * - del(id) -> id
 * - put(id, collection, reference, details) -> id | updated_at
 * - set(id, key, value) -> id | updated_at
 */

enum Nature {
    Debit,  // 01
    Credit, // 11
    Nobit,  // 00
}
struct Id();
struct At();
struct Concept();
struct Quantity();

struct Observer {
    id: Id,
    observables: Vec<Id>,
    data: Option<String>,
}

struct Observable {
    id: Id,
    record: Vec<Entry>,
    data: Option<String>,
}

struct Entry {
    id: Id,
    // Immutable
    concept: Concept,
    nature: Nature,
    quantity: Quantity,
    // Mutable
    collection: Option<Id>,
    reference: Option<Id>,
    details: Option<String>,
    // UTC Datetime
    created_at: At, // Immutable
    updated_at: At, // Mutable
}
