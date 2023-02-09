#[derive(Clone)]
pub(crate) struct Particle{
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) px: f32,
    pub(crate) py: f32,
    pub(crate) ax: f32,
    pub(crate) ay: f32,
    pub(crate) color: (u8, u8, u8)
}

impl Particle {
    pub(crate) fn new(x: f32, y: f32, ax: f32, ay: f32, color: (u8, u8, u8)) -> Self{
        Self {
            x,
            y,
            px: x,
            py: y,
            ax,
            ay,
            color,
        }
    }
}

pub(crate) struct World {
    pub(crate) cells: Vec<Vec<Vec<Particle>>>,
    pub(crate) width: usize,
    pub(crate) height: usize
}

impl World {
    pub(crate) fn new(w: usize, h: usize) -> Self{
        Self {
            cells: vec![vec![vec![];h];w],
            width: w,
            height: h
        }
    }

    pub(crate) fn add_particle(&mut self, p: Particle) {
        self.cells[(p.x as usize).max(0).min(self.width-1)][(p.y as usize).max(0).min(self.height-1)].push(p)
    }
}