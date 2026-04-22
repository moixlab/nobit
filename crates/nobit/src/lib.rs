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
    at: At,
    concept: Concept,
    nature: Nature,
    quantity: Quantity,
    collection: Option<Id>,
    reference: Option<Id>,
    details: Option<String>,
}
