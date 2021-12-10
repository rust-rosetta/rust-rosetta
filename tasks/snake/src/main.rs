// Implemented smooth (per-pixel) animation on Win32 API (tested on Windows 7)
// Used winsafe - a library of safe rust-bindings for Win32 GUI: young, but with convenient links to docs.microsoft.com from doc and src.

#![windows_subsystem = "windows"]

use rand::{thread_rng, Rng};
use std::{cell::RefCell, rc::Rc};
use winsafe::{co, gui, prelude::*, COLORREF, HPEN, SIZE};

const STEP: i32 = 3; // px, offset per frame. STEP and FPS determine the smoothness and speed of the animation.
const FPS: u32 = 90;
const CELL: i32 = 21; // px, game grid (logical step). Will be aligned by STEP
const SNAKE_W: i32 = 20; // px,
const ROUNDING: SIZE = SIZE::new(SNAKE_W / 2, SNAKE_W / 2);
/// side of a square fields in CELLs
const FIELD_W: i32 = 20;
/// count of STEPs per CELL
const RATIO: i32 = CELL / STEP;
/// total field width (with overlap for collisions) in STEPs
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
    /// IDs of 5 fillable rect (bg, tail, body, food, head); id_rect = y * TW + x (where x, y: nSTEPs)
    id_r: [i32; 5],
    /// \[ids_rect] where id_rect = y * TW + x (where x, y: nSTEPs)
    snake: Vec<i32>,
    /// gap in STEPs between animation and logic cell (negative - remove tail)
    gap: i32,
    dir: Direction,
    ordered_dir: Direction,
}
impl Context {
    fn new(wnd: gui::WindowMain, len: usize) -> Self {
        Self {
            wnd,
            id_r: [FIELD_W / 2 * RATIO; 5],
            snake: vec![FIELD_W / 2 * RATIO; 1.max(len as i32 - RATIO) as usize],
            gap: -1,
            dir: Start,
            ordered_dir: S,
        }
    }
}

fn main() {
    let [bg, tail, body, food, head] = [0usize, 1, 2, 3, 4];
    let brushes = winsafe::COLORREF::new_array(&[
        (0x00, 0x50, 0x90), // color bg
        (0x00, 0xF0, 0xA0), // color tail
        (0x00, 0xF0, 0xA0), // color body
        (0xFF, 0x50, 0x00), // color food
        (0xFF, 0xFF, 0x00), // color head
    ])
    .map(|c| winsafe::HBRUSH::CreateSolidBrush(c).unwrap());

    let wnd = gui::WindowMain::new(gui::WindowMainOpts {
        title: "Snake - Start: Space, then press W-A-S-D".to_string(),
        size: winsafe::SIZE::new(FIELD_W * RATIO * STEP, FIELD_W * RATIO * STEP),
        ex_style: co::WS_EX::CLIENTEDGE,
        class_bg_brush: brushes[bg],
        ..Default::default()
    });

    let context = Rc::new(RefCell::new(Context::new(wnd.clone(), 0)));

    wnd.on().wm_key_down({
        let context = Rc::clone(&context);
        move |k| {
            let bt = k.char_code as u8;
            if b" ADSW\x71".contains(&bt) {
                let mut ctx = context.borrow_mut();
                match (ctx.dir, bt) {
                    (Start, b' ' | 0x71) => {
                        *ctx = Context::new(
                            ctx.wnd.clone(),
                            if bt == b' ' { ctx.snake.len() } else { 0 },
                        );
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
            ctx.snake.insert(0, new_h);
            if ctx.gap < 0 {
                ctx.id_r[bg] = ctx.snake.pop().unwrap();
                ctx.id_r[tail] = *ctx.snake.last().unwrap();
            }
            ctx.gap -= ctx.gap.signum();
            if ctx.gap == 0 {
                ctx.dir = ctx.ordered_dir;
                let itr = ctx.snake.iter().step_by(RATIO as usize);
                if new_h == ctx.id_r[food] {
                    let mut snake_cells: Vec<_> = itr.collect();
                    let len = snake_cells.len();
                    let title = if len == cells.len() {
                        ctx.id_r[food] = 0;
                        format!("Snake - EATEN ALL: {} !!!", len - 1)
                    } else {
                        snake_cells.sort();
                        ctx.id_r[food] = *cells
                            .iter()
                            .filter(|i| snake_cells.binary_search(i).is_err())
                            .nth(thread_rng().gen_range(0..1.max(cells.len() - len)))
                            .unwrap_or(&0);
                        format!("Snake - Eaten: {}.", len - 1)
                    };
                    ctx.wnd.hwnd().SetWindowText(&title)?;
                    ctx.gap = RATIO;
                } else if cells.binary_search(&new_h).is_err() || itr.skip(1).any(|&j| j == new_h) {
                    let title =
                        ctx.wnd.hwnd().GetWindowText()? + "  Restarting: F2 (with save - Space)";
                    ctx.wnd.hwnd().SetWindowText(&title)?;
                    ctx.wnd.hwnd().KillTimer(1)?;
                    ctx.dir = Start;
                } else {
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
        hdc.SelectObjectPen(HPEN::CreatePen(co::PS::NULL, 0, COLORREF::new(0, 0, 0))?)?;
        for (&id_rect, &brush) in ctx.id_r.iter().zip(&brushes) {
            hdc.SelectObjectBrush(brush)?;
            let left = id_rect % TW * STEP - (STEP * RATIO + SNAKE_W) / 2;
            let top = id_rect / TW * STEP - (STEP * RATIO + SNAKE_W) / 2;
            hdc.RoundRect(
                winsafe::RECT {
                    left,
                    top,
                    right: left + SNAKE_W,
                    bottom: top + SNAKE_W,
                },
                ROUNDING,
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
