use super::Vector3D;

impl Vector3D {
    /// if av + bc + cv = 0v; mu = av.bv + bv.cv + cv.av; given |av|, |bv| and |cv|
    ///
    /// a + b + c = 0v;
    /// 
    /// a.a + a.b + a.c = 0;
    /// a.b + a.c = - (a.a) = -(|a|*|a|) -- (1)
    /// 
    /// a.b + b.c = - (b.b) = -(|b|*|b|) -- (2)
    /// 
    /// a.c + b.c = n? -- (3)
    /// 
    /// (1) + (2) + (3) -> 2(a.b + b.c + a.c) = (-(|a|^2)) + (-(|b|^2) + n?) -- (4)
    /// 
    /// 2mu = (4) -> mu = (4) / 2
    pub fn evaluate_mu(vec_1_mag: f64, vec_2_mag: f64, vec_3_mag: f64) -> f64 {
        -0.5 * (vec_1_mag.powi(2) + vec_2_mag.powi(2) + vec_3_mag.powi(2))
    }
}