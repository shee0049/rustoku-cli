use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use curl::easy::Easy;
use std::io::Read;
use std::process;

const IP_ADDR: &'static str = "";
const MENU: &'static str = "
╔═══════════════════════════════╦═════════════════════════╗\r
║ Power On       p              ║ Replay          r       ║\r
║ Power Off      P              ║ Info/Settings   I       ║\r
║ Back           <Backspace>    ║ Rewind          <left>  ║\r
║ Home           R              ║ Fast-Fwd        <right> ║\r
║ Left           h              ║ Play/Pause      <Space> ║\r
║ Down           j              ║ Input mode      i       ║\r
║ Up             k              ║ Volume Up       <up>    ║\r
║ Right          l              ║ Volume Down     <down>  ║\r
║ Ok/Enter       <Enter>        ║ Volume Mute     m       ║\r
║                               ║                         ║\r
║ Exit           q              ║                         ║\r
╚═══════════════════════════════╩═════════════════════════╝\n\r"; 
const INPUT_MENU: &'static str = "
╔═══════════════════════════════╗\r
║ Input Mode...                 ║\r
║                               ║\r
║ Leave Input Mode    <Esc>     ║\r
║ Enter Text          <Enter>   ║\r
║ Exit                Ctrl<c>   ║\r
╚═══════════════════════════════╝\n\r"; 

macro_rules! clear_screen {
    () => {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide,
            termion::clear::All
        ).unwrap();
    };
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    clear_screen!();

    stdout.write(format!("Connected to roku at: {}\r", IP_ADDR).as_bytes()).unwrap();
    stdout.write(MENU.as_bytes()).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('j')  => handle_press("down"),
            Key::Char('k')  => handle_press("up"),
            Key::Char('l')  => handle_press("right"),
            Key::Char('h')  => handle_press("left"),
            Key::Char('\n') => handle_press("select"),
            Key::Char('H')  => handle_press("home"),
            Key::Char('r')  => handle_press("instantreplay"),
            Key::Char('m')  => handle_press("volumemute"),
            Key::Char('p')  => handle_press("poweron"),
            Key::Char('P')  => handle_press("poweroff"),
            Key::Char('I')  => handle_press("info"),
            Key::Char(' ')  => handle_press("play"),
            Key::Backspace  => handle_press("Back"),
            Key::Up         => handle_press("volumeup"),
            Key::Down       => handle_press("volumedown"),
            Key::Left       => handle_press("rev"),
            Key::Right      => handle_press("fwd"),
            Key::Char('q')  => process::exit(1),
            Key::Ctrl('c')  => process::exit(1),
            Key::Char('i')  => input_mode(),
            _ => (),
        }
    }
}


fn handle_press( _action: &str){

    let mut data = "".as_bytes();
    let mut easy = Easy::new();

    let req = format!("http://{}:8060/keypress/{}", IP_ADDR, _action);

    easy.url(&req).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap(); 
}

fn input_mode(){
    
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    clear_screen!();

    stdout.write(format!("Connected to roku at: {}\r", IP_ADDR).as_bytes()).unwrap();
    stdout.write(INPUT_MENU.as_bytes()).unwrap();
   
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => main(),
            Key::Ctrl('c')  => process::exit(1),
            Key::Char('\n') => handle_press("enter"),
            Key::Backspace  => handle_press("backspace"),
            Key::Char(c)    => handle_key(c),
            _ => (),
        }      
    }
}

fn handle_key(_key: char){
    
    let mut data = "".as_bytes();
    let mut easy = Easy::new();

    let req = if _key == ' ' { 
        format!("http://{}:8060/keypress/Lit_{}", IP_ADDR, "%20") 
    } else { 
        format!("http://{}:8060/keypress/Lit_{}", IP_ADDR, _key) 
    }; 

    easy.url(&req).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();
    
    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().unwrap(); 
} 
