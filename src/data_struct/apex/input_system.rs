use named_constants::named_constants;
use serde::{Deserialize, Serialize};

#[named_constants]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
#[allow(clippy::upper_case_acronyms)]
pub enum InputSystem {
    KEY_NONE,
    KEY_0 = 0,
    KEY_1 = 1,
    KEY_2 = 2,
    KEY_3 = 3,
    KEY_4 = 4,
    KEY_5 = 5,
    KEY_6 = 6,
    KEY_7 = 7,
    KEY_8 = 8,
    KEY_9 = 9,
    KEY_A = 10,
    KEY_B = 11,
    KEY_C = 12,
    KEY_D = 13,
    KEY_E = 14,
    KEY_F = 15,
    KEY_G = 16,
    KEY_H = 17,
    KEY_I = 18,
    KEY_J = 19,
    KEY_K = 20,
    KEY_L = 21,
    KEY_M = 22,
    KEY_N = 23,
    KEY_O = 24,
    KEY_P = 25,
    KEY_Q = 26,
    KEY_R = 27,
    KEY_S = 28,
    KEY_T = 29,
    KEY_U = 30,
    KEY_V = 31,
    KEY_W = 32,
    KEY_X = 33,
    KEY_Y = 34,
    KEY_Z = 35,
    KEY_PAD_0 = 36,
    KEY_PAD_1 = 37,
    KEY_PAD_2 = 38,
    KEY_PAD_3 = 39,
    KEY_PAD_4 = 40,
    KEY_PAD_5 = 41,
    KEY_PAD_6 = 42,
    KEY_PAD_7 = 43,
    KEY_PAD_8 = 44,
    KEY_PAD_9 = 45,
    KEY_PAD_DIVIDE = 46,
    KEY_PAD_MULTIPLY = 47,
    KEY_PAD_MINUS = 48,
    KEY_PAD_PLUS = 49,
    KEY_PAD_ENTER = 50,
    KEY_PAD_DECIMAL = 51,
    KEY_LBRACKET = 52,
    KEY_RBRACKET = 53,
    KEY_SEMICOLON = 54,
    KEY_APOSTROPHE = 55,
    KEY_BACKQUOTE = 56,
    KEY_COMMA = 57,
    KEY_PERIOD = 58,
    KEY_SLASH = 59,
    KEY_BACKSLASH = 60,
    KEY_MINUS = 61,
    KEY_EQUAL = 62,
    KEY_ENTER = 63,
    KEY_SPACE = 64,
    KEY_BACKSPACE = 65,
    KEY_TAB = 66,
    KEY_CAPSLOCK = 67,
    KEY_NUMLOCK = 68,
    KEY_ESCAPE = 69,
    KEY_SCROLLLOCK = 70,
    KEY_INSERT = 71,
    KEY_DELETE = 72,
    KEY_HOME = 73,
    KEY_END = 74,
    KEY_PAGEUP = 75,
    KEY_PAGEDOWN = 76,
    KEY_BREAK = 77,
    KEY_LSHIFT = 78,
    KEY_RSHIFT = 79,
    KEY_LALT = 80,
    KEY_RALT = 81,
    KEY_LCONTROL = 82,
    KEY_RCONTROL = 83,
    KEY_LWIN = 84,
    KEY_RWIN = 85,
    KEY_APP = 86,
    KEY_UP = 87,
    KEY_LEFT = 88,
    KEY_DOWN = 89,
    KEY_RIGHT = 90,
    KEY_F1 = 91,
    KEY_F2 = 92,
    KEY_F3 = 93,
    KEY_F4 = 94,
    KEY_F5 = 95,
    KEY_F6 = 96,
    KEY_F7 = 97,
    KEY_F8 = 98,
    KEY_F9 = 99,
    KEY_F10 = 100,
    KEY_F11 = 101,
    KEY_F12 = 102,
    KEY_CAPSLOCKTOGGLE = 103,
    KEY_NUMLOCKTOGGLE = 104,

    MOUSE_LEFT = 107,
    MOUSE_RIGHT = 108,
    MOUSE_MIDDLE = 109,
    MOUSE_4 = 110, // side down

    MOUSE_5 = 111, // side up

    // XBox 360
    KEY_XBUTTON_UP = 242, // POV buttons

    KEY_XBUTTON_RIGHT = 243,
    KEY_XBUTTON_DOWN = 244,
    KEY_XBUTTON_LEFT = 245,

    KEY_XBUTTON_A = 114,
    KEY_XBUTTON_B = 115,
    KEY_XBUTTON_X = 116,
    KEY_XBUTTON_Y = 117,

    KEY_XBUTTON_LEFT_SHOULDER = 118,
    KEY_XBUTTON_RIGHT_SHOULDER = 119,

    KEY_XBUTTON_BACK = 120,
    KEY_XBUTTON_START = 121,

    KEY_XBUTTON_STICK1 = 122, // left stick down
    KEY_XBUTTON_STICK2 = 123, // right stick down

    KEY_XBUTTON_LTRIGGER = 125, // ZAXIS POSITIVE
    KEY_XBUTTON_RTRIGGER = 126, // ZAXIS NEGATIVE
}



