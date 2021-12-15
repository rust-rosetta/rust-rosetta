// Implemented smooth (per-pixel) animation on Win32 API (tested on Windows 7)
// Used winsafe - a library of safe rust-bindings for Win32 GUI: young, but with convenient links to docs.microsoft.com from doc and src.

#![cfg(windows)]

use rand::Rng;
use std::{cell::RefCell, rc::Rc};
use winsafe::{co, gui, prelude::*, COLORREF, HBRUSH, HPEN, SIZE};

const STEP: i32 = 3; // px, offset per frame. STEP and FPS determine the smoothness and speed of the animation.
const FPS: u32 = 90;
const CELL: i32 = 21; // px, game grid (logical step). Will be aligned by STEP
const FIELD_W: i32 = 20; // width of the square field in CELLs
const SNAKE_W: i32 = 20; // px
const ROUNDING: SIZE = SIZE::new(SNAKE_W / 2, SNAKE_W / 2);

const RATIO: i32 = CELL / STEP;
const START_CELL: i32 = FIELD_W / 2 * RATIO;
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
    snake: Vec<i32>, // [ids_rect] where id_rect = y * TW + x (where x, y: nSTEPs)
    id_r: [i32; 6],  // ID 6 rectangles to color in next frame (bg, tail, turn, body, food, head)
    gap: i32,        // gap in STEPs between animation and logic cell (negative - remove tail)
    dir: Direction,
    ordered_dir: Direction,
}
impl Context {
    fn new(wnd: gui::WindowMain, len: usize) -> Self {
        Self {
            wnd,
            snake: vec![START_CELL; 1.max(len as i32 - RATIO) as usize],
            id_r: [START_CELL; 6],
            gap: -1,
            dir: Start,
            ordered_dir: S,
        }
    }
}

pub fn main() {
    let [bg, tail, turn, body, food, head] = [0usize, 1, 2, 3, 4, 5];
    let mut colors = [(0x00, 0xF0, 0xA0); 6]; // color tail, turn, body
    colors[bg] = (0x00, 0x50, 0x90);
    colors[food] = (0xFF, 0x50, 0x00);
    colors[head] = (0xFF, 0xFF, 0x00);
    let brushes = COLORREF::new_array(&colors).map(|c| HBRUSH::CreateSolidBrush(c).unwrap());

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
            let mut ctx = context.borrow_mut();
            match (ctx.dir, k.char_code as u8) {
                (Start, bt @ (b' ' | 113)) => {
                    let len = ctx.snake.len();
                    *ctx = Context::new(ctx.wnd.clone(), if bt == b' ' { len } else { 0 });
                    ctx.wnd.hwnd().InvalidateRect(None, true)?;
                    ctx.wnd.hwnd().SetTimer(1, 1000 / FPS, None)?;
                }
                (W | S, bt @ (b'A' | b'D')) => ctx.ordered_dir = if bt == b'A' { A } else { D },
                (A | D, bt @ (b'S' | b'W')) => ctx.ordered_dir = if bt == b'S' { S } else { W },
                _ => (),
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
                ctx.id_r[turn] = ctx.snake[ctx.snake.len().saturating_sub(1 + RATIO as usize / 2)];
            }
            ctx.gap -= ctx.gap.signum();
            if ctx.gap == 0 {
                ctx.dir = ctx.ordered_dir;
                let hw = ctx.wnd.hwnd();
                let mut scells: Vec<_> = ctx.snake.iter().step_by(RATIO as usize).collect();
                let eat = new_h == ctx.id_r[food];
                if !eat && (cells.binary_search(&new_h).is_err() || scells[1..].contains(&&new_h)) {
                    hw.SetWindowText(&(hw.GetWindowText()? + "  Restart: F2 (with save - Space)"))?;
                    hw.KillTimer(1)?;
                    ctx.dir = Start;
                } else if eat || ctx.id_r[food] == 0 && ctx.id_r[tail] != START_CELL {
                    let len = scells.len() + if eat { 1 } else { 0 };
                    if len == cells.len() {
                        hw.SetWindowText(&format!("Snake - EATEN ALL: {} !!!", len - 2))?
                    } else {
                        hw.SetWindowText(&format!("Snake - Eaten: {}.", len - 2))?
                    }
                    if ctx.id_r[tail] == START_CELL || len == cells.len() {
                        ctx.id_r[food] = 0; // hide food if not all of the saved snake has come out or everything is eaten
                    } else {
                        scells.sort();
                        ctx.id_r[food] = *(cells.iter())
                            .filter(|i| scells.binary_search(i).is_err())
                            .nth(rand::thread_rng().gen_range(0..cells.len() - scells.len()))
                            .unwrap();
                    }
                }
                ctx.gap = if eat { RATIO } else { -RATIO }
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
