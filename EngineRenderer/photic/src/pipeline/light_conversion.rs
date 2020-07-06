//NOTE: Needs testing based on: https://i.stack.imgur.com/bnwoo.png
//Gotten from: https://stackoverflow.com/questions/1472514/convert-light-frequency-to-rgb
pub fn wavelength_to_rgb(wavelength: f32) -> [f32; 3] {
    let mut red: f32 = 0.0;
    let mut green: f32 = 0.0;
    let mut blue: f32 = 0.0;

    if wavelength >= 380.0 && wavelength < 440.0 {
        red = -(wavelength - 440.0) / (440.0 - 380.0);
        green = 0.0;
        blue = 1.0;
    } else if wavelength >= 440.0 && wavelength < 490.0 {
        red = 0.0;
        green = (wavelength - 440.0) / (490.0 - 440.0);
        blue = 1.0;
    } else if wavelength >= 490.0 && wavelength < 510.0 {
        red = 0.0;
        green = 1.0;
        blue = -(wavelength - 510.0) / (510.0 - 490.0);
    } else if wavelength >= 510.0 && wavelength < 580.0 {
        red = (wavelength - 510.0) / (580.0 - 510.0);
        green = 1.0;
        blue = 0.0;
    } else if wavelength >= 580.0 && wavelength < 645.0 {
        red = 1.0;
        green = -(wavelength - 645.0) / (645.0 - 580.0);
        blue = 0.0;
    } else if wavelength >= 645.0 && wavelength < 781.0 {
        red = 1.0;
        green = 0.0;
        blue =  0.0;
    } else {
        //Not in the visible spectrum
        red = 0.0;
        green = 0.0;
        blue = 0.0;
    }

    //Let the intensity fall off near the vision limits
    let mut factor: f32 = 0.0;

    if wavelength >= 380.0 && wavelength < 420.0 {
        factor = 0.3 + 0.7 * (wavelength - 380.0) / (420.0 - 380.0);
    } else if wavelength >= 420.0 && wavelength < 701.0 {
        factor = 1.0;
    } else if wavelength >= 701.0 && wavelength < 781.0 {
        factor = 0.3 + 0.7 * (780.0 - wavelength) / (780.0 - 700.0);
    }

    [
        if red == 0.0 { 0.0 } else { (red * factor).powf(2.2) },
        if green == 0.0 { 0.0 } else { (green * factor).powf(2.2) },
        if blue == 0.0 { 0.0 } else { (blue * factor).powf(2.2) }
    ]
}

//TODO: Temperature (Kelvin) to RGB

//Conversion between units
pub fn degrees_to_steradians(degrees: f32) -> f32 {
    degrees / 90.0 * std::f32::consts::PI
}

pub fn steradians_to_degrees(steradians: f32) -> f32 {
    steradians / std::f32::consts::PI * 90.0
}

pub fn lumen_to_candela(lumen: f32) -> f32 {
    lumen / (4.0 * std::f32::consts::PI)
}

pub fn candela_to_lumen(candela: f32, steradians: f32) -> f32 {
    candela * steradians
}

pub fn candela_to_lux(candela: f32, distance: f32) -> f32 {
    candela / (distance * distance)
}
