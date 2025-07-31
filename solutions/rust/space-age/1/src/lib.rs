#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn orbital_period() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (Self::orbital_period() * 31_557_600f64)
    }
}

macro_rules! planet {
    ($planet:ident, $period:literal) => {
        pub struct $planet;
        impl Planet for $planet {
            fn orbital_period() -> f64 {
                $period
            }
        }
    }
}

planet!(Mercury, 000.24084670);
planet!(Venus  , 000.61519726);
planet!(Earth  , 001.00000000);
planet!(Mars   , 001.88081580);
planet!(Jupiter, 011.86261500);
planet!(Saturn , 029.44749800);
planet!(Uranus , 084.01684600);
planet!(Neptune, 164.79132000);

