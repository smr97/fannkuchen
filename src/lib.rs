pub mod fannkuchh_adaptive;
pub mod fannkuchh_original;
pub mod fannkuchh_rayon;
pub mod fannkuchh_sequential;
pub use crate::fannkuchh_adaptive::fannkuchh_adaptive;
pub use crate::fannkuchh_original::fannkuchh_fastest;
pub use crate::fannkuchh_rayon::fannkuchh_rayon;
pub use crate::fannkuchh_sequential::fannkuchh_sequential;
