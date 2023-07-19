#[inline(always)]
pub fn pretty_print(x: f64, y: f64, z: f64) -> String {
    if [x, y, z] == [0.0; 3] {
        return "0.0i+0.0j+0.0k".to_string();
    }

    let fx: String = {
        if x == 1.0 {
            "i".to_string()
        } else if x == -1.0 {
            "-i".to_string()
        } else if x > 0.0 {
            format!("{}i", x)
        } else if x < 0.0 {
            format!("{:+}i", x)
        } else {
            "".to_string()
        }
    };

    let fy: String = {
        if y == 1.0 {
            if x != 0.0 {
                "+j".to_string()
            } else {
                "j".to_string()
            }
        } else if y == -1.0 {
            "-j".to_string()             
        } else if y == 0.0 {
            "".to_string()
        } else {
            format!("{:+}j", y)
        }
    };

    let fz: String = {
        if z == 1.0 {
            if (x != 0.0) | (y != 0.0) {
                "+k".to_string()
            } else {
                "k".to_string()
            }
        } else if z == -1.0 {
            "-k".to_string()             
        } else if z == 0.0 {
            "".to_string()
        } else {
            format!("{:+}k", z)
        }
    };

    fx + &fy + &fz
}