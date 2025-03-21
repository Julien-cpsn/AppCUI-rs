/*
    Copyright (c) 2016 Jesse 'Jeaye' Wilkerson

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.

    Except as contained in this notice, the name(s) of the above copyright
    holders shall not be used in advertising or otherwise to promote the
    sale, use or other dealings in this Software without prior written
    authorization.
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use libc::{c_short, c_int};
use super::constants::*;

#[derive(Debug, Clone, Copy)]
pub enum CURSOR_VISIBILITY
{
  CURSOR_INVISIBLE = 0,
  CURSOR_VISIBLE,
  CURSOR_VERY_VISIBLE
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub struct MEVENT { pub id: c_short, pub x: c_int, pub y: c_int, pub z: c_int, pub bstate: mmask_t}

#[derive(Debug)]
pub enum WchResult {
    KeyCode(i32),
    Char(winttype),
}

#[derive(Debug)]
#[repr(i32)]
pub enum LcCategory {
    all = LC_ALL,
    collate = LC_COLLATE,
    ctype = LC_CTYPE,
    monetary = LC_MONETARY,
    numeric = LC_NUMERIC,
    time = LC_TIME,
    messages = LC_MESSAGES,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(crate) struct SMALL_RECT {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(super) struct COORD {
    pub X: i16,
    pub Y: i16,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, Debug)]
pub(super) struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: u16,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD,
}