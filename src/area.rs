pub trait areaType {
    fn areaTypesPrint(&self);
}

pub struct rectangle {
    pub width: u64,
    pub long: u64,
}

pub struct squareBox {
    pub width: u64,
    pub long: u64,
}

impl areaType for rectangle {
    fn areaTypesPrint(&self) {
        println!("area is（长方形的面积是） {}", self.long * self.width);
    }
}
impl areaType for squareBox {
    fn areaTypesPrint(&self) {
        println!("area is（正方形的面积是） {}", self.long * self.width);
    }
}

pub fn areaPrint<T: areaType>(item: T) {
    item.areaTypesPrint();
}
