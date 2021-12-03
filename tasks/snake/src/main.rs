// Implemented smooth (per-pixel) animation on Win32 API (tested on Windows 7)
// Used winsafe - a library of safe rust-bindings for Win32 GUI: young, but with convenient links to docs.microsoft.com from doc and src.

#![windows_subsystem = "windows"]

use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};
use winsafe::{co, gui, prelude::*};

const FPS: u32 = 90;
const STEP: i32 = 3; // px, offset per frame. FPS and STEP determine the speed and smoothness of the animation.

const _CELL: i32 = 21; // px, game grid (logical step). Will be aligned by STEP to CELL
const SNAKE_W: i32 = 16; // px,
const FIELD_W: i32 = 20; // side of a square fields in CELLs

const RATIO: i32 = _CELL / STEP; // CELL in STEPs
const CELL: i32 = STEP * RATIO; // pxs
const WW: i32 = (FIELD_W + 2) * RATIO; // total width (with overlap for collisions) in STEPs
const OFFSET: i32 = (-CELL - SNAKE_W) / 2; // px
#[derive(Clone, Copy)]
enum Direction {
    Start = 0,
    A = -1,
    D = 1,
    W = -WW as isize,
    S = WW as isize,
}
use Direction::*;

struct Context {
    wnd: gui::WindowMain,
    idr: [i32; 5],  // ids of filled rectangles
    lsnk: Vec<i32>, // snake logics:[id_cell, id_cell, ...]
    asnk: Vec<i32>, // snake animation:[id_rectan, id_rectan, ...]
    gap: i32,       // gap in STEPs between logic and animation (negative - remove tail)
    ldir: Direction,
    adir: Direction,
}
impl Context {
    fn new(wnd: gui::WindowMain) -> Self {
        Self {
            wnd,
            idr: [FIELD_W / 2 * RATIO; 5],
            lsnk: Vec::with_capacity((FIELD_W * FIELD_W) as usize),
            asnk: Vec::with_capacity((FIELD_W * FIELD_W * RATIO) as usize),
            gap: 1,
            ldir: S,
            adir: Start,
        }
    }
}

fn main() {
    const FIELD: usize = 0;
    const TAIL: usize = 1;
    const BODY: usize = 2;
    const FOOD: usize = 3;
    const HEAD: usize = 4;
    let brushes = winsafe::COLORREF::new_array(&[
        (0xFF, 0xF9, 0xD0), // color:FIELD
        (0x00, 0xB0, 0xA0), // color:TAIL
        (0x00, 0xB0, 0xA0), // color:BODY
        (0xFF, 0x20, 0x20), // color:FOOD
        (0x20, 0x60, 0x00), // color:HEAD
    ])
    .map(|c| winsafe::HBRUSH::CreateSolidBrush(c).unwrap());

    let wnd = gui::WindowMain::new(gui::WindowMainOpts {
        title: "Snake: press 'A-W-S-D'".to_string(),
        size: winsafe::SIZE::new(FIELD_W * CELL, FIELD_W * CELL),
        ex_style: co::WS_EX::CLIENTEDGE,
        class_bg_brush: brushes[FIELD],
        ..Default::default()
    });

    let context = Rc::new(RefCell::new(Context::new(wnd.clone())));

    wnd.on().wm_key_down({
        let context = Rc::clone(&context);
        move |k| {
            let bt = k.char_code as u8;
            if b"ADSW".contains(&bt) {
                let mut ctx = context.borrow_mut();
                match (ctx.adir, bt) {
                    (Start, _) => {
                        *ctx = Context::new(ctx.wnd.clone());
                        ctx.wnd.hwnd().InvalidateRect(None, true)?;
                        ctx.wnd.hwnd().SetTimer(1, 1000 / FPS, None)?;
                    }
                    (W | S, b'A') => ctx.ldir = A,
                    (W | S, b'D') => ctx.ldir = D,
                    (A | D, b'S') => ctx.ldir = S,
                    (A | D, b'W') => ctx.ldir = W,
                    _ => (),
                }
            }
            Ok(())
        }
    });

    wnd.on().wm_timer(1, {
        let context = Rc::clone(&context);
        let cells: Vec<i32> = (1..=FIELD_W)
            .flat_map(|y| (1..=FIELD_W).map(move |x| (y * WW + x) * RATIO))
            .collect();
        move || {
            let mut ctx = context.borrow_mut();
            let head = ctx.idr[HEAD] + ctx.adir as i32;
            ctx.idr[BODY] = ctx.idr[HEAD];
            ctx.idr[HEAD] = head;
            ctx.asnk.insert(0, head);
            if ctx.gap < 0 {
                ctx.idr[FIELD] = ctx.asnk.pop().unwrap();
                ctx.idr[TAIL] = *ctx.asnk.last().unwrap();
            }
            ctx.gap -= ctx.gap.signum();
            if ctx.gap == 0 {
                ctx.lsnk.insert(0, head);
                ctx.adir = ctx.ldir;
                if head == ctx.idr[FOOD] {
                    ctx.wnd
                        .hwnd()
                        .SetWindowText(&format!("Snake; eaten {} pieces ", ctx.lsnk.len() - 1))?;
                    ctx.idr[FOOD] = *cells
                        .iter()
                        .filter(|&i| !ctx.lsnk.contains(i))
                        .nth(thread_rng().gen_range(0..cells.len() - ctx.lsnk.len()))
                        .unwrap();
                    ctx.gap = RATIO;
                } else if cells.binary_search(&head).is_err() || ctx.lsnk[1..].contains(&head) {
                    ctx.wnd.hwnd().KillTimer(1)?;
                    ctx.adir = Start;
                } else {
                    ctx.lsnk.pop();
                    ctx.gap = -RATIO;
                }
            }
            ctx.wnd.hwnd().InvalidateRect(None, false)?; // call .wm_paint()
            Ok(())
        }
    });

    wnd.on().wm_paint(move || {
        let ctx = context.borrow();
        let mut ps = winsafe::PAINTSTRUCT::default();
        let hdc = ctx.wnd.hwnd().BeginPaint(&mut ps)?;
        for (&idrect, &brush) in ctx.idr.iter().zip(&brushes) {
            let left = idrect % WW * STEP + OFFSET;
            let top = idrect / WW * STEP + OFFSET;
            hdc.FillRect(
                winsafe::RECT {
                    left,
                    top,
                    right: left + SNAKE_W,
                    bottom: top + SNAKE_W,
                },
                brush,
            )?;
        }
        ctx.wnd.hwnd().EndPaint(&ps);
        Ok(())
    });

    if let Err(e) = wnd.run_main(None) {
        winsafe::HWND::NULL
            .MessageBox(&e.to_string(), "Uncaught error", co::MB::ICONERROR)
            .unwrap();
    }
}
