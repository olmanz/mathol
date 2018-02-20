/// Trait for calculating the area of a two-dimensional object
/// # Usage
/// ```
/// use mathol::geometrics::traits::Area;
/// ```
pub trait Area {
    fn get_area(self) -> f64;
}

/// Trait for calculating the perimeter of a two-dimensional object
/// # Usage
/// ```
/// use mathol::geometrics::traits::Perimeter;
/// ```
pub trait Perimeter {
    fn get_perimeter(self) -> f64;
}

/// Trait for calculating the internal diagonal of two- or three-dimensional objects
/// # Usage
/// ```
/// use mathol::geometrics::traits::Diagonal;
/// ```
pub trait Diagonal {
    fn get_diagonal(self) -> f64;
}

/// Trait for calculating the height of an object
/// # Usage
/// ```
/// use mathol::geometrics::traits::Height;
/// ```
pub trait Height {
    fn get_height(self) -> f64;
}

/// Trait for calculating the volume of a three-dimensional object
/// # Usage
/// ```
/// use mathol::geometrics::traits::Volume;
/// ```
pub trait Volume {
    fn get_volume(self) -> f64;
}

/// Trait for calculating the surface of a three-dimensional object
/// # Usage
/// ```
/// use mathol::geometrics::traits::Surface;
/// ```
pub trait Surface {
    fn get_surface(self) -> f64;
}

/// Trait for calculating the lateral surface of some three-dimensional objects
/// # Usage
/// ```
/// use mathol::geometrics::traits::Lateral;
/// ```
pub trait Lateral {
    fn get_lateral(self) -> f64;
}