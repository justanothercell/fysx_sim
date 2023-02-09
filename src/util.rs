pub(crate) fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    if s == 0.0 {
        r = v;
        g = v;
        b = v;
    }
    else {
        let h = if h == 360.0 {
            0.0
        } else {
            h / 60.0
        };

        let i = h.trunc() as i32;
        let f = h - i as f32;

        let p = v * (1.0 - s);
        let q = v * (1.0 - (s * f));
        let t = v * (1.0 - (s * (1.0 - f)));

        match i {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            _ => {
                r = v;
                g = p;
                b = q;
            }
        }

    }
    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

pub(crate) unsafe fn mutate<T>(thing: &T) -> &mut T {
    &mut*(thing as *const T as *mut T)
}