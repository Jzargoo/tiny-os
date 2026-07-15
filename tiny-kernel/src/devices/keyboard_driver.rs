pub enum KeyCode {
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z
}

pub struct Modifiers {
    shift: bool,
    ctrl: bool,
    alt: bool
}

pub struct KeyboardDriver {
    pressed_key: Key
}

pub struct Key {
    active_key_code: KeyCode,
    modifiers: Modifiers
}