use crate::container::Space;
use crate::container::{IDError, IDKind};
use crate::events::{Events, EventsConclusion, EventsTrigger};
use crate::kbd_decode::{decode_ki, read_ki, KbdEvent};
use crate::Term;
use std::collections::HashMap;
use std::io::{StdinLock, StdoutLock};

use crate::ID;

struct EventsQueue<'a, 'b, P, T, IE, R>
where
    IE: Events<P, T>,
    T: EventsTrigger,
    R: EventsConclusion,
{
    queue: HashMap<ID<'a>, Vec<EventsDocument<'b, P, T, IE, R>>>,
}

// WARN: this wont work
// will NOT allow for different Ts or Rs, etc.,
struct EventsDocument<'a, P, T, IE, R>
where
    IE: Events<P, T>,
    T: EventsTrigger,
    R: EventsConclusion,
{
    id: ID<'a>,
    active: bool,
    asyncness: bool,
    f: fn(IE, T) -> R,
    phantom: std::marker::PhantomData<P>,
}

impl<'a, P, T, IE, R> EventsDocument<'a, P, T, IE, R>
where
    IE: Events<P, T>,
    T: EventsTrigger,
    R: EventsConclusion,
{
    fn new(asyncness: bool, id: ID<'a>, f: fn(IE, T) -> R) -> Self {
        Self {
            active: true,
            asyncness,
            id,
            f,
            phantom: std::marker::PhantomData::<P>,
        }
    }
}

pub struct Commissioner;

impl Commissioner {
    pub fn authorize_id(ik: IDKind) -> Result<u8, IDError> {
        match ik {
            IDKind::TextInput => Ok(0), // checks for self last TextId and returns the next one
            IDKind::TextNE => Ok(0),    // checks for self last TextId and returns the next one
            IDKind::Events => Ok(0),
            IDKind::Component => Ok(0),
            IDKind::BufferImage => Err(IDError::ProgramIsUnique),
        }
    }

    pub fn approve_space(space: Space<usize>) -> bool {
        true
    }

    pub async fn process<'t1, 't2>(ke: &KbdEvent, term: &mut Term<'t1, 't2>) {}

    pub fn clear(writer: &mut StdoutLock) {}

    pub fn render(writer: &mut StdoutLock, term: &mut Term) {}
}

struct InnerLogic;

impl Commissioner {
    fn bind<'eq1, 'eq2, P, T, R, IE>(
        events: &mut EventsQueue<'eq1, 'eq2, P, T, IE, R>,
        f: fn(IE, T) -> R,
        eid: ID<'eq2>,
        id: ID<'eq1>,
    ) where
        // F: Fn(IE, T) -> R,
        IE: Events<P, T>,
        T: EventsTrigger,
        R: EventsConclusion,
    {
        let doc = EventsDocument::new(false, eid, f);

        if events.queue.contains_key(&id) {
            events.queue.get_mut(&id).unwrap().push(doc);
        } else {
            events.queue.insert(id, vec![doc]);
        }
    }

    fn bind_async<'eq1, 'eq2, P, T, R, IE>(
        events: &mut EventsQueue<'eq1, 'eq2, P, T, IE, R>,
        f: fn(IE, T) -> R,
        eid: ID<'eq2>,
        id: ID<'eq1>,
    ) where
        // F: Fn(IE, T) -> R,
        IE: Events<P, T>,
        T: EventsTrigger,
        R: EventsConclusion,
    {
        let doc = EventsDocument::new(true, eid, f);

        if events.queue.contains_key(&id) {
            events.queue.get_mut(&id).unwrap().push(doc);
        } else {
            events.queue.insert(id, vec![doc]);
        }
    }

    fn release() {}
}

// TODO:
// events for
// space logic,
// id logic,
// term, components, input and noneditable logic,
// have to be implemented by this crate

type comr = Commissioner;

async fn ragout<'a, 'b, P, T, IE, R>(
    reader: &mut StdinLock<'static>,
    input: &mut Vec<u8>,
    term: &mut Term<'a, 'b>,
    writer: &mut StdoutLock<'static>,
) where
    IE: Events<P, T>,
    T: EventsTrigger,
    R: EventsConclusion,
{
    let fps = 60;
    let refresh = 1000 / fps;

    loop {
        let input = decode_ki(read_ki(reader, input));

        std::thread::sleep(std::time::Duration::from_millis(refresh));
        comr::process(&input, term).await;
        comr::clear(writer);
        comr::render(writer, term);
    }
}