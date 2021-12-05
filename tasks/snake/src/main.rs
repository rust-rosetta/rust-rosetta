// Implemented smooth (per-pixel) animation on Win32 API (tested on Windows 7)
// Used winsafe - a library of safe rust-bindings for Win32 GUI: young, but with convenient links to docs.microsoft.com from doc and src.

#![windows_subsystem = "windows"]

use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};
use winsafe::{co, gui, prelude::*};

const FPS: u32 = 90;
const STEP: i32 = 3; // px, offset per frame. FPS and STEP determine the speed and smoothness of the animation.

const CELL: i32 = 21; // px, game grid (logical step). Will be aligned by STEP
const SNAKE_W: i32 = 16; // px,
/// side of a square fields in CELLs
const FIELD_W: i32 = 20;
/// count of STEPs per CELL
const RATIO: i32 = CELL / STEP;
/// total width (with overlap for collisions) in STEPs
const TW: i32 = (FIELD_W + 2) * RATIO;
#[derive(Clone, Copy)]
#[repr(i32)]
enum Direction {
    Start = 0,
    A = -1,
    D = 1,
    W = -TW,
    S = TW,
}
use Direction::*;

struct Context {
    wnd: gui::WindowMain,
    /// IDs of fillable rect (bg, tail, body, food, head)
    id_r: [i32; 5],
    /// animation snake: \[id_rect, id_rect, ...]
    a_snk: Vec<i32>,
    /// logic snake: \[id_cell as id_rect, ...]  
    l_snk: Vec<i32>,
    /// gap in STEPs between  animation and logic (negative - remove tail)
    gap: i32,
    dir: Direction,
    ordered_dir: Direction,
}
impl Context {
    fn new(wnd: gui::WindowMain) -> Self {
        Self {
            wnd,
            id_r: [FIELD_W / 2 * RATIO; 5],
            a_snk: Vec::with_capacity((FIELD_W * FIELD_W * RATIO) as usize),
            l_snk: Vec::with_capacity((FIELD_W * FIELD_W) as usize),
            gap: 1,
            dir: Start,
            ordered_dir: S,
        }
    }
}

fn main() {
    let [bg, tail, body, food, head] = [0usize, 1, 2, 3, 4];
    let brushes = winsafe::COLORREF::new_array(&[
        (0xFF, 0xF9, 0xD0), // color bg
        (0x00, 0xB0, 0xA0), // color tail
        (0x00, 0xB0, 0xA0), // color body
        (0xFF, 0x20, 0x20), // color food
        (0x20, 0x60, 0x00), // color head
    ])
    .map(|c| winsafe::HBRUSH::CreateSolidBrush(c).unwrap());

    let wnd = gui::WindowMain::new(gui::WindowMainOpts {
        title: "Snake - Start: Space, then press W-A-S-D".to_string(),
        size: winsafe::SIZE::new(FIELD_W * RATIO * STEP, FIELD_W * RATIO * STEP),
        ex_style: co::WS_EX::CLIENTEDGE,
        class_bg_brush: brushes[bg],
        ..Default::default()
    });

    let context = Rc::new(RefCell::new(Context::new(wnd.clone())));

    wnd.on().wm_key_down({
        let context = Rc::clone(&context);
        move |k| {
            let bt = k.char_code as u8;
            if b"ADSW ".contains(&bt) {
                let mut ctx = context.borrow_mut();
                match (ctx.dir, bt) {
                    (Start, b' ') => {
                        *ctx = Context::new(ctx.wnd.clone());
                        ctx.wnd.hwnd().InvalidateRect(None, true)?;
                        ctx.wnd.hwnd().SetTimer(1, 1000 / FPS, None)?;
                    }
                    (W | S, b'A') => ctx.ordered_dir = A,
                    (W | S, b'D') => ctx.ordered_dir = D,
                    (A | D, b'S') => ctx.ordered_dir = S,
                    (A | D, b'W') => ctx.ordered_dir = W,
                    _ => (),
                }
            }
            Ok(())
        }
    });

    wnd.on().wm_timer(1, {
        let context = Rc::clone(&context);
        let cells: Vec<i32> = (1..=FIELD_W)
            .flat_map(|y| (1..=FIELD_W).map(move |x| (y * TW + x) * RATIO))
            .collect();
        move || {
            let mut ctx = context.borrow_mut();
            let new_h = ctx.id_r[head] + ctx.dir as i32;
            ctx.id_r[body] = ctx.id_r[head];
            ctx.id_r[head] = new_h;
            ctx.a_snk.insert(0, new_h);
            if ctx.gap < 0 {
                ctx.id_r[bg] = ctx.a_snk.pop().unwrap();
                ctx.id_r[tail] = *ctx.a_snk.last().unwrap();
            }
            ctx.gap -= ctx.gap.signum();
            if ctx.gap == 0 {
                ctx.l_snk.insert(0, new_h);
                ctx.dir = ctx.ordered_dir;
                if new_h == ctx.id_r[food] {
                    let title = format!("Snake - Eaten {}", ctx.l_snk.len() - 1);
                    ctx.wnd.hwnd().SetWindowText(&title)?;
                    ctx.id_r[food] = *cells
                        .iter()
                        .filter(|&i| !ctx.l_snk.contains(i))
                        .nth(thread_rng().gen_range(0..cells.len() - ctx.l_snk.len()))
                        .unwrap();
                    ctx.gap = RATIO;
                } else if cells.binary_search(&new_h).is_err() || ctx.l_snk[1..].contains(&new_h) {
                    let title = ctx.wnd.hwnd().GetWindowText()? + ". Restarting: Space";
                    ctx.wnd.hwnd().SetWindowText(&title)?;
                    ctx.wnd.hwnd().KillTimer(1)?;
                    ctx.dir = Start;
                } else {
                    ctx.l_snk.pop();
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
        for (&id_rect, &brush) in ctx.id_r.iter().zip(&brushes) {
            let left = id_rect % TW * STEP - (STEP * RATIO + SNAKE_W) / 2;
            let top = id_rect / TW * STEP - (STEP * RATIO + SNAKE_W) / 2;
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
