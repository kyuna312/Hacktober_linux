//! NyanNix ASCII Animation

const FRAMES: [&str; 6] = [
    r#"
 +      o     +              o    +
     +             o     +       +
 o          +
     o  +           +        +
 +        o     o       +        o
 ~-_-_-_-_-_-_-,------,      o
 _-_-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-_-~|__( ^ .^)  +     +
 _-_-_-_-_-_-_-""  ""
     N y a n N i x   v 0.1
 +      o         o   +       o
     +         +
 o        o         o      o     +
     o           +
 +      +     o        o      +    "#,
    r#"
     o       +      o    +      o
 +      +           +        +
     o  +           o     +       o
 o        +     o        +
     +         +          o      +
 -~-_-_-_-_-_-,------,      o
 _-_-_-_-_-_-_|   /\_/\
 -_-_-_-_-_-_~|__( > .^)  +     +
 _-_-_-_-_-_-_""  ""
   ~ N y a n N i x ~  v 0.1
     o         +           o     +
 +         +        +
 o     o         o      o     +
     +         +          +
 o      o         o           +    "#,
    r#"
 o      +      o    +      o      +
     o           +        +
 +      o     +              o
     +             o     +       +
 o          +
 _-~-_-_-_-_-,------,      o
 _-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-~|__( ^ .-)  +     +
 _-_-_-_-_-_-""  ""
   * N y a n N i x *  v 0.1
 +      o         o   +       o
     +         +
 o        o         o      o     +
     o           +
 +      +     o        o      +    "#,
    r#"
     o       +      o    +      o
 +      +           +        +
     o  +           o     +       o
 o        +     o        +
     +         +          o      +
 _-_-~-_-_-_-,------,      o
 _-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-~|__( - .-)  +     +
 _-_-_-_-_-_-""  ""
   . N y a n N i x .  v 0.1
     o         +           o     +
 +         +        +
 o     o         o      o     +
     +         +          +
 o      o         o           +    "#,
    r#"
 +      o     +              o
     +             o     +       +
 o          +
     o  +           +        +
 +        o     o       +        o
 _-_-_-~-_-_-,------,      o
 _-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-~|__( ^ o^)  +     +
 _-_-_-_-_-_-""  ""
   ~ N y a n N i x ~  v 0.1
 +      o         o   +       o
     +         +
 o        o         o      o     +
     o           +
 +      +     o        o      +    "#,
    r#"
     o       +      o    +      o
 +      +           +        +
     o  +           o     +       o
 o        +     o        +
     +         +          o      +
 _-_-_-_-~-_-,------,      o
 _-_-_-_-_-_-|   /\_/\
 -_-_-_-_-_-~|__( o .o)  +     +
 _-_-_-_-_-_-""  ""
   * N y a n N i x *  v 0.1
     o         +           o     +
 +         +        +
 o     o         o      o     +
     +         +          +
 o      o         o           +    "#,
];

static mut FRAME_INDEX: usize = 0;
static mut RAINBOW_COLORS: [&str; 6] = [
    "\x1B[1;31m", // Red
    "\x1B[1;33m", // Yellow
    "\x1B[1;32m", // Green
    "\x1B[1;36m", // Cyan
    "\x1B[1;34m", // Blue
    "\x1B[1;35m", // Magenta
];

/// Get the next frame of the animation
pub fn next_frame() -> &'static str {
    unsafe {
        let frame = FRAMES[FRAME_INDEX];
        FRAME_INDEX = (FRAME_INDEX + 1) % FRAMES.len();
        frame
    }
}

/// Get the current color
pub fn current_color() -> &'static str {
    unsafe { RAINBOW_COLORS[FRAME_INDEX] }
}
