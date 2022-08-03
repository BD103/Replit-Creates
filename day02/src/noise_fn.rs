use noise::{self, NoiseFn};

pub struct NoiseFunctions {
    selected: NoiseTypes,
    billow: noise::Billow,
    fbm: noise::Fbm,
    simplex: noise::OpenSimplex,
    perlin: noise::Perlin,
}

impl NoiseFunctions {
    pub fn get_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        use NoiseTypes::*;

        let point = [x, y, z];

        match self.selected {
            Billow => self.billow.get(point),
            Fbm => self.fbm.get(point),
            Simplex => self.simplex.get(point),
            Perlin => self.perlin.get(point),
        }
    }

    pub fn selected(&self) -> NoiseTypes {
        self.selected
    }

    pub fn select(&mut self, noise_type: NoiseTypes) {
        self.selected = noise_type;
    }
}

impl Default for NoiseFunctions {
    fn default() -> Self {
        NoiseFunctions {
            selected: NoiseTypes::Perlin,
            billow: Default::default(),
            fbm: Default::default(),
            simplex: Default::default(),
            perlin: Default::default(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoiseTypes {
    Billow,
    Fbm,
    Simplex,
    Perlin,
}

impl NoiseTypes {
    pub fn next(&self) -> Self {
        #[allow(unreachable_patterns)]
        match self {
            Self::Billow => Self::Fbm,
            Self::Fbm => Self::Simplex,
            Self::Simplex => Self::Perlin,
            Self::Perlin => Self::Billow,
            _ => Self::Perlin,
        }
    }
}
