mod area;
mod sum;
enum trafficLight {
    Red,
    Green,
    Yellow,
}

trait Time {
    fn time(&self) -> u8;
}

impl Time for trafficLight {
    fn time(&self) -> u8 {
        match self {
            trafficLight::Red => 60,
            trafficLight::Green => 60,
            trafficLight::Yellow => 10,
        }
    }
}

fn main() {
    // let yellow = trafficLight::Yellow;
    // let red = trafficLight::Red;
    // let green = trafficLight::Green;

    // println!("red（红色） time is {:?}", red.time());
    // println!("green（绿色） time is {:?}", green.time());
    // println!("yellow(黄色) time is {:?}", yellow.time());

    let list = [1, 2, 3, 4, 5, 4294967295, 1999999999, 1999999999];
    let list1 = [1, 2, 3, 4, 5, 6, 7, 8];
    let k = sum::total(&list);
    let k1 = sum::total(&list1);
    match k {
        Some(c) => println!("{c}"),
        None => {
            println!("溢出错误None")
        }
    }
    match k1 {
        Some(c) => println!("和为{c}"),
        None => {
            println!("None")
        }
    }

    let rectangle1 = area::rectangle {
        width: 10,
        long: 20,
    };
    let squareBox1 = area::squareBox {
        width: 20,
        long: 20,
    };
    area::areaPrint(rectangle1);
    area::areaPrint(squareBox1);
}
