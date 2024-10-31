use ragout::components::{ComponentTree as CT, Container, Term, Text};
use ragout::console::{
    cooked_mode, enter_alternate_screen, leave_alternate_screen, raw_mode, workers,
};
use ragout::frames;
use ragout::space::{border::Border, padding::Padding, Area, Pos};

use std::io::Write;

const ZPOS: Pos = Pos::Value(0);

fn main() {
    let mut tree = CT::init();

    let term = tree.term_mut(0).unwrap();

    println!("{:?}", term);

    let res0 = term.container(
        &[0, 0],
        Pos::Start,
        Pos::Start,
        ZPOS,
        Area::Fill,
        Border::polyform('1', '2', '3', '4', '|', '-'),
        Padding::None,
    );
    println!("{:?}", term.containers.len());

    let res1 = term.input(
        &[0, 0, 0],
        Pos::Start,
        Pos::Start,
        ZPOS,
        Area::Values { w: 43, h: 16 },
        Border::uniform('i'),
        Padding::None,
    );
    println!("{:?}", term.containers.get(&[0, 0]).unwrap().texts.len());

    let res2 = term.noedit(
        &[0, 0, 1],
        Pos::End,
        Pos::End,
        ZPOS,
        Area::Values { w: 43, h: 16 },
        Border::uniform('n'),
        Padding::None,
        &vec![],
    );
    println!("{:?}", term.containers.get(&[0, 0]).unwrap().texts.len());
    println!("{:?}", res2);

    assert!(res0.is_ok());
    assert!(res1.is_ok());
    assert!(res2.is_ok());

    let (_, mut writer) = workers();

    let ts = raw_mode();
    enter_alternate_screen(&mut writer);

    term.clear(&mut writer);
    term.render(&mut writer);
    term.render_cursor(&mut writer);

    let mut counter = 0;
    loop {
        frames(60);
        counter += 1;
        // keep display for 6 seconds then quit loop
        if counter == 360 {
            break;
        }
    }

    cooked_mode(ts);
    leave_alternate_screen(&mut writer);
}
