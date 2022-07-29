

use std::fs::File;
use std::io::{Read, Write};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use winput::{Vk, Button};
use std::convert::From;


// only left and right mouse buttons have been covered
// but you can add as many buttons as you wish
#[derive(Serialize, Deserialize, Debug)]
pub enum MouseButton { Left, Right , Other}
impl MouseButton{
    pub fn new(button: Button) -> Self {
        match button {
            Button::Left => Self::Left,
            Button::Right => Self::Right,
            _ => Self::Other
        }
    }
}
// only press and release modifiers have been covered
// but you can add as many as you wish
#[derive(Serialize, Deserialize, Debug)]
pub enum Modifier{Press, Release}

// only arrow keys have been covered
// but you can add as many as you wish
#[derive(Serialize, Deserialize, Debug)]
// pub enum Key{LeftArrow, RightArrow, UpArrow, DownArrow, Other}
#[derive(FromPrimitive)]
pub enum Key {
    /// Left mouse button
    ///
    /// **VK_LBUTTON** = 0x01
    MouseLeft = 0x01,
    /// Right mouse button
    ///
    /// **VK_RBUTTON** = 0x02
    MouseRight = 0x02,
    /// Control-break processing
    ///
    /// **CANCEL** = 0x03
    Cancel = 0x03,
    /// Middle mouse button (three-button mouse)
    ///
    /// **VK_MBUTTON** = 0x04
    MouseMiddle = 0x04,
    /// X1 mouse button
    ///
    /// **VK_XBUTTON1** = 0x05
    MouseX1 = 0x05,
    /// X2 mouse button
    ///
    /// **VK_XBUTTON1** = 0x06
    MouseX2 = 0x06,
    /// BACKSPACE key
    ///
    /// **VK_BACK** = 0x08
    Backspace = 0x08,
    /// TAB key
    ///
    /// **VK_TAB** = 0x09
    Tab = 0x09,
    /// CLEAR key
    ///
    /// **VK_CLEAR** = 0x0c
    Clear = 0x0c,
    /// ENTER key
    ///
    /// **VK_RETURN** = 0x0d
    Enter = 0x0d,
    /// SHIFT key
    ///
    /// **VK_SHIFT** = 0x10
    Shift = 0x10,
    /// CTRL key
    ///
    /// **VK_CONTROL** = 0x11
    Control = 0x11,
    /// ALT key
    ///
    /// **VK_MENU** = 0x12
    Alt = 0x12,
    /// PAUSE key
    ///
    /// **VK_PAUSE** = 0x13
    Pause = 0x13,
    /// CAPS LOCK key
    ///
    /// **VK_CAPITAL** = 0x14
    CapsLock = 0x14,
    /// IME Kana mode & IME Hangul mode
    ///
    /// **VK_KANA** = **VK_HANGUL** = 0x15
    Kana = 0x15,
    /// IME On
    ///
    /// **VK_IME_ON** = 0x16
    ImeOn = 0x16,
    /// IME Junja mode
    ///
    /// **VK_JUNJA** = 0x17
    Junja = 0x17,
    /// IME final mode
    ///
    /// **VK_FINAL** = 0x18
    Final = 0x18,
    /// IME Kanji mode & IME Hanja mode
    ///
    /// **VK_KANJI** = **VK_HANJA** = 0x19
    Kanji = 0x19,
    /// IME Off
    ///
    /// **VK_Ime** = 0x1a
    ImeOff = 0x1a,
    /// ESC key
    ///
    /// **VK_ESCAPE** = 0x1b
    Escape = 0x1b,
    /// IME convert
    ///
    /// **VK_CONVERT** = 0x1c
    Convert = 0x1c,
    /// IME nonconvert
    ///
    /// **VK_NONCONVERT** = 0x1d
    NonConvert = 0x1d,
    /// IME accept
    ///
    /// **VK_ACCEPT** = 0x1e
    Accept = 0x1e,
    /// IME mode change request
    ///
    /// **VK_MODECHANGE** = 0x1f
    ModeChange = 0x1f,
    /// SPACEBAR
    ///
    /// **VK_SPACE** = 0x20
    Space = 0x20,
    /// PAGE UP key
    ///
    /// **VK_PRIOR** = 0x21
    PageUp = 0x21,
    /// PAGE DOWN key
    ///
    /// **VK_NEXT** = 0x22
    PageDown = 0x22,
    /// END key
    ///
    /// **VK_END** = 0x23
    End = 0x23,
    /// HOME key
    ///
    /// **VK_HOME** = 0x24
    Home = 0x24,
    /// LEFT ARROW key
    ///
    /// **VK_LEFT** = 0x25,
    LeftArrow = 0x25,
    /// UP ARROW key
    ///
    /// **VK_UP** = 0x26
    UpArrow = 0x26,
    /// RIGHT ARROW key
    ///
    /// **VK_RIGHT** = 0x27
    RightArrow = 0x27,
    /// DOWN ARROW key
    ///
    /// **VK_DOWN** = 0x28
    DownArrow = 0x28,
    /// SELECT key
    ///
    /// **VK_SELECT** = 0x29
    Select = 0x29,
    /// PRINT key
    ///
    /// **VK_PRINT** = 0x2a
    Print = 0x2a,
    /// EXECUTE key
    ///
    /// **VK_EXECUTE** = 0x2b
    Execute = 0x2b,
    /// PRINT SCREEN key
    ///
    /// **VK_SNAPSHOT** = 0x2c
    PrintScreen = 0x2c,
    /// INS key
    ///
    /// **VK_INSERT** = 0x2d
    Insert = 0x2d,
    /// DEL key
    ///
    /// **VK_DELETE** = 0x2e
    Delete = 0x2e,
    /// HELP key
    ///
    /// **VK_HELP** = 0x2f,
    Help = 0x2f,
    /// 0 key
    _0 = b'0' as isize,
    /// 1 key
    _1 = b'1' as isize,
    /// 2 key
    _2 = b'2' as isize,
    /// 3 key
    _3 = b'3' as isize,
    /// 4 key
    _4 = b'4' as isize,
    /// 5 key
    _5 = b'5' as isize,
    /// 6 key
    _6 = b'6' as isize,
    /// 7 key
    _7 = b'7' as isize,
    /// 8 key
    _8 = b'8' as isize,
    /// 9 key
    _9 = b'9' as isize,
    /// A key
    A = b'A' as isize,
    /// B key
    B = b'B' as isize,
    /// C key
    C = b'C' as isize,
    /// D key
    D = b'D' as isize,
    /// E key
    E = b'E' as isize,
    /// F key
    F = b'F' as isize,
    /// G key
    G = b'G' as isize,
    /// H key
    H = b'H' as isize,
    /// I key
    I = b'I' as isize,
    /// J key
    J = b'J' as isize,
    /// K key
    K = b'K' as isize,
    /// L key
    L = b'L' as isize,
    /// M key
    M = b'M' as isize,
    /// N key
    N = b'N' as isize,
    /// O key
    O = b'O' as isize,
    /// P key
    P = b'P' as isize,
    /// Q key
    Q = b'Q' as isize,
    /// R key
    R = b'R' as isize,
    /// S key
    S = b'S' as isize,
    /// T key
    T = b'T' as isize,
    /// U key
    U = b'U' as isize,
    /// V key
    V = b'V' as isize,
    /// W key
    W = b'W' as isize,
    /// X key
    X = b'X' as isize,
    /// Y key
    Y = b'Y' as isize,
    /// Z key
    Z = b'Z' as isize,
    /// Left Windows key (Natural keyboard)
    ///
    /// **VK_LWIN** = 0x5b,
    LeftWin = 0x5b,
    /// Right Windows key (Natural keyboard)
    ///
    /// **VK_RWIN** = 0x5c
    RightWin = 0x5c,
    /// Applications key (Natural keyboard)
    ///
    /// **VK_APPS** = 0x5d
    Apps = 0x5d,
    /// Computer Sleep key
    ///
    /// **VK_SLEEP** = 0x5f
    Sleep = 0x5f,
    /// Numeric keypad 0 key
    ///
    /// **VK_NUMPAD0** = 0x60
    Numpad0 = 0x60,
    /// Numeric keypad 1 key
    ///
    /// **VK_NUMPAD1** = 0x61
    Numpad1 = 0x61,
    /// Numeric keypad 2 key
    ///
    /// **VK_NUMPAD2** = 0x62
    Numpad2 = 0x62,
    /// Numeric keypad 3 key
    ///
    /// **VK_NUMPAD3** = 0x63
    Numpad3 = 0x63,
    /// Numeric keypad 4 key
    ///
    /// **VK_NUMPAD4** = 0x64
    Numpad4 = 0x64,
    /// Numeric keypad 5 key
    ///
    /// **VK_NUMPAD5** = 0x65
    Numpad5 = 0x65,
    /// Numeric keypad 6 key
    ///
    /// **VK_NUMPAD6** = 0x66
    Numpad6 = 0x66,
    /// Numeric keypad 7 key
    ///
    /// **VK_NUMPAD7** = 0x67
    Numpad7 = 0x67,
    /// Numeric keyapd 8 key
    ///
    /// **VK_NUMPAD8** = 0x68
    Numpad8 = 0x68,
    /// Numeric keypad 9 key
    ///
    /// **VK_NUMPAD9** = 0x69
    Numpad9 = 0x69,
    /// Multiply key
    ///
    /// **VK_MULTIPLY** = 0x6a
    Multiply = 0x6a,
    /// Add key
    ///
    /// **VK_ADD** = 0x6b,
    Add = 0x6b,
    /// Separator key
    ///
    /// **VK_SEPARATOR** = 0x6c,
    Separator = 0x6c,
    /// Subtract key
    ///
    /// **VK_SUBTRACT** = 0x6d
    Subtract = 0x6d,
    /// Decimal key
    ///
    /// **VK_DECIMAL** = 0x6e
    Decimal = 0x6e,
    /// Divide key
    ///
    /// **VK_DIVIDE** = 0x6f
    Divide = 0x6f,
    /// F1 key
    ///
    /// **VK_F1** = 0x70
    F1 = 0x70,
    /// F2 key
    ///
    /// **VK_F2** = 0x71
    F2 = 0x71,
    /// F3 key
    ///
    /// **VK_F3** = 0x72
    F3 = 0x72,
    /// F4 key
    ///
    /// **VK_F4** = 0x73
    F4 = 0x73,
    /// F5 key
    ///
    /// **VK_F5** = 0x74
    F5 = 0x74,
    /// F6 key
    ///
    /// **VK_F6** = 0x75
    F6 = 0x75,
    /// F7 key
    ///
    /// **VK_F7** = 0x76
    F7 = 0x76,
    /// F8 key
    ///
    /// **VK_F8** = 0x77,
    F8 = 0x77,
    /// F9 key
    ///
    /// **VK_F9** = 0x78,
    F9 = 0x78,
    /// F10 key
    ///
    /// **VK_F10** = 0x79,
    F10 = 0x79,
    /// F11 key
    ///
    /// **VK_F11** = 0x7a
    F11 = 0x7a,
    /// F12 key
    ///
    /// **VK_F12** = 0x7b
    F12 = 0x7b,
    /// F13 key
    ///
    /// **VK_F13** = 0x7c
    F13 = 0x7c,
    /// F14 key
    ///
    /// **VK_F14** = 0x7d
    F14 = 0x7d,
    /// F15 key
    ///
    /// **VK_F15** = 0x7e
    F15 = 0x7e,
    /// F16 key
    ///
    /// **VK_F16** = 0x7f
    F16 = 0x7f,
    /// F17 key
    ///
    /// **VK_F17** = 0x80
    F17 = 0x80,
    /// F18 key
    ///
    /// **VK_F18** = 0x81,
    F18 = 0x81,
    /// F19 key
    ///
    /// **VK_F19** = 0x82,
    F19 = 0x82,
    /// F20 key
    ///
    /// **VK_F20** = 0x83,
    F20 = 0x83,
    /// F21 key
    ///
    /// **VK_F21** = 0x84,
    F21 = 0x84,
    /// F22 key
    ///
    /// **VK_F22** = 0x85,
    F22 = 0x85,
    /// F23 key
    ///
    /// **VK_F23** = 0x86,
    F23 = 0x86,
    /// F24 key
    ///
    /// **VK_F24** = 0x87,
    F24 = 0x87,
    /// NUM LOCK key
    ///
    /// **VK_NUMLOCK** = 0x90
    Numlock = 0x90,
    /// SCROLL LOCK key
    ///
    /// **VK_SCROLL** = 0x91
    Scroll = 0x91,
    /// Left SHIFT key
    ///
    /// **VK_LSHIFT** = 0xa0
    LeftShift = 0xa0,
    /// Right SHIFT key
    ///
    /// **VK_RSHIFT** = 0xa1
    RightShift = 0xa1,
    /// Left CONTROL key
    ///
    /// **VK_LCONTROL** = 0xa2
    LeftControl = 0xa2,
    /// Right CONTROL key
    ///
    /// **VK_RCONTROL** = 0xa3
    RightControl = 0xa3,
    /// Left MENU key
    ///
    /// **VK_LMENU** = 0xa4
    LeftMenu = 0xa4,
    /// Right MENU key
    ///
    /// **VK_RMENU** = 0xa5
    RightMenu = 0xa5,
    /// Browser Back key
    ///
    /// **VK_BROWSER_BACK** = 0xa6
    BrowserBack = 0xa6,
    /// Browser Forward key
    ///
    /// **VK_BROWSER_FORWARD** = 0xa7
    BrowserForward = 0xa7,
    /// Browser Refresh key
    ///
    /// **VK_BROWSER_REFRESH** = 0xa8
    BrowserRefresh = 0xa8,
    /// Browser Stop key
    ///
    /// **VK_BROWSER_STOP** = 0xa9
    BrowserStop = 0xa9,
    /// Browser Search key
    ///
    /// **VK_BROWSER_SEARCH** = 0xaa
    BrowserSearch = 0xaa,
    /// Browser Favorites key
    ///
    /// **VK_BROWSER_FAVORITES** = 0xab
    BrowserFavorites = 0xab,
    /// Browser Start and Home key
    ///
    /// **VK_BROWSER_HOME** = 0xac
    BrowserHome = 0xac,
    /// Volume Mute key
    ///
    /// **VK_VOLUME_MUTE** = 0xad
    VolumeMute = 0xad,
    /// Volume Down key
    ///
    /// **VK_VOLUME_DOWN** = 0xae
    VolumeDown = 0xae,
    /// Volume Up key
    ///
    /// **VK_VOLUME_UP** = 0xaf
    VolumeUp = 0xaf,
    /// Next Track key
    ///
    /// **VK_MEDIA_NEXT_TRACK** = 0xb0
    NextTrack = 0xb0,
    /// Prev Track key
    ///
    /// **VK_MEDIA_PREV_TRACK** = 0xb1
    PrevTrack = 0xb1,
    /// Stop Media key
    ///
    /// **VK_MEDIA_STOP** = 0xb2
    MediaStop = 0xb2,
    /// Play/Pause Media key
    ///
    /// **VK_MEDIA_PLAY_PAUSE** = 0xb3
    MediaPlayPause = 0xb3,
    /// Start Mail key
    ///
    /// **VK_LAUNCH_MAIL** = 0xb4
    StartMail = 0xb4,
    /// Select Media key
    ///
    /// **VK_LAUNCH_MEDIA_SELECT** = 0xb5
    SelectMedia = 0xb5,
    /// Start Application 1 key
    ///
    /// **VK_LAUNCH_APP1** = 0xb6
    StartApp1 = 0xb6,
    /// Start Application 2 key
    ///
    /// **VK_LAUNCH_APP2** = 0xb7
    StartApp2 = 0xb7,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `;:` key.
    ///
    /// **VK_OEM_1** = 0xba
    Oem1 = 0xba,
    /// For any country/region, the `+` key.
    ///
    /// **VK_OEM_PLUS** = 0xbb
    Plus = 0xbb,
    /// For any country/region, the `,` key.
    ///
    /// **VK_OEM_COMMA** = 0xbc
    Comma = 0xbc,
    /// For any country/region, the `-` key.
    ///
    /// **VK_OEM_MINUS** = 0xbd
    Minus = 0xbd,
    /// For any country/region, the `.` key.
    ///
    /// **VK_OEM_PERIOD** = 0xbe
    Period = 0xbe,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `\?` key.
    ///
    /// **VK_OEM_2** = 0xbf
    Oem2 = 0xbf,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the ``~` key.
    ///
    /// **VK_OEM_3** = 0xc0
    Oem3 = 0xc0,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `[{` key.
    ///
    /// **VK_OEM_4** = 0xdb
    Oem4 = 0xdb,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `\|` key.
    ///
    /// **VK_OEM_5** = 0xdc
    Oem5 = 0xdc,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `]}` key.
    ///
    /// **VK_OEM_6** = 0xdd
    Oem6 = 0xdd,
    /// Used for miscellaneous characters; it can vary by keyboard. For the US standard
    /// keyboard, the `'"` key.
    ///
    /// **VK_OEM_7** = 0xde
    Oem7 = 0xde,
    /// Used for miscellaneous characters; it can vary by keyboard.
    ///
    /// **VK_OEM_8** = 0xdd
    Oem8 = 0xdf,
    /// Either the angle bracket key or the backslash key on the RT 102-key keyboard.
    ///
    /// **VK_OEM_102** = 0xe2
    Oem102 = 0xe2,
    /// IME PROCESS key
    ///
    /// **VK_PROCESSKEY** = 0xe5
    ImeProcess = 0xe5,
    /// Attn key
    ///
    /// **VK_ATTN** = 0xf6
    Attn = 0xf6,
    /// CrSel key
    ///
    /// **VK_CRSEL** = 0xf7
    CrSel = 0xf7,
    /// ExSel key
    ///
    /// **VK_EXSEL** = 0xf8
    ExSel = 0xf8,
    /// Erase EOF key
    ///
    /// **VK_EREOR** = 0xf9
    EraseEof = 0xf9,
    /// Play key
    ///
    /// **VK_PLAY** = 0xfa
    Play = 0xfa,
    /// Zoom key
    ///
    /// **VK_ZOOM** = 0xfb
    Zoom = 0xfb,
    /// PA1 key
    ///
    /// **VK_PA1** = 0xfd
    Pa1 = 0xfd,
    /// Clear key
    ///
    /// **VK_OEM_CLEAR** = 0xfe
    OemClear = 0xfe,
}


impl From<Vk> for Key {
    #[inline(always)]
    fn from(vk: Vk) -> Self {
        let val: Self = num::FromPrimitive::from_u8(vk.into_u8()).unwrap();
        val
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinates{
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseEvent{
    // fields you mentioned has been added
    // but you can add and remove fields as you wish
    pub(crate) button: MouseButton,
    pub(crate) modifier: Modifier,
    pub(crate) event_time: SystemTime,
    pub(crate) event_coordinate: Coordinates, }

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyboardEvent{
    // fields you mentioned has been added
    // but you can add and remove fields as you wish
    pub(crate) key: Key,
    pub(crate) modifier: Modifier,
    pub(crate) event_time: SystemTime, }

#[derive(Serialize, Deserialize, Debug)]
pub struct EventDatabase{
    // this is the actual database that keeps track of events with their time and other details
    pub(crate) mouse_database: Vec<MouseEvent>,
    pub(crate) key_database: Vec<KeyboardEvent>
}


impl EventDatabase {
    pub fn new() -> Self {
        EventDatabase{
            mouse_database: vec![],
            key_database: vec![]
        }
    }

    // just add a mouse event to the database
    pub fn add_mouse_event(&mut self, event: MouseEvent) {
        self.mouse_database.push(event);
    }
    // just add keyboard event to database
    pub fn add_keyboard_event(&mut self, event: KeyboardEvent) {
        self.key_database.push(event);
    }

    // this function saves the data to a file,
    // so for persistent database, make sure to save database to a file
    pub fn save_database(&self, filename: String){
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    // this function is used to create EventDatabase from file
    // use only file created by save_database method to save database
    pub fn load_database(filename: String) -> Self{
        let mut file_result = File::open(filename);
        match file_result {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let deserialized: Self = serde_json::from_str(&contents).unwrap();
                deserialized
            }
            Err(e) => {
                println!("No database found or Error Reading Database");
                Self::new()
            }
        }

    }

    // this  function auto saves database to file when the program ends
    // but in case of force quit or panic, this don't works so make sure
    // to call save_database manually
    pub fn drop(&mut self) {
        self.save_database("database".to_string());
    }
}