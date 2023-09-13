#[derive(Debug)]
struct Triangle {
    cat1: f64,
    cat2: f64,

}
//методы структуры
impl Triangle {
    fn find_area(&self) -> f64 {
        (self.cat1 * self.cat2) / 2.0
    }
    fn find_perimeter(&self) -> f64 {
        0.5 * (self.cat1 + self.cat2)
    }
    //связанная функция
    fn create_triangle(cat: f64) -> Triangle {
        Triangle {
            cat1: cat,
            cat2: cat
        }
    }
    fn is_equal(&self, arr:f64) -> bool {
        self.find_area() < arr
    }
    fn main() {
        let triangle = Triangle {
            cat1: 7.0,
            cat2: 9.0,
        };
        let triangle1 = Triangle {
            cat1: 5.0,
            cat2: 4.0,
        };

        let triangle_isc = Triangle::create_triangle(7.0);

        println!("{} and {}, {}, {}", triangle.find_area(), triangle.find_perimeter()
            ,triangle_isc.find_area(), triangle_isc.find_perimeter());

        let result = triangle1.is_equal(triangle_isc.find_area());
        if result == true {
            println!("Треугольник поместился");
        }else {
            println!("Треугольник не поместился");
        }
    }
}

