use raster::{ Color, Image };
use std::f64::consts::PI;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color() -> Color {
        let r = rand::random_range(0..=255);
        let g = rand::random_range(0..=255);
        let b = rand::random_range(0..=255);
        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// struct of point implementation
#[derive(Clone,Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

// implement it
// done with it

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y}
    }
    pub fn random(width: i32, height: i32) -> Self {
        let y = rand::random_range(0..height);
        let x = rand::random_range(0..width);
        Point::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, Point::color());
    }
}

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point,
    pub color: Color,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a: a,
            b: b,
            color: Line::color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let a_x = rand::random_range(0..width);
        let a_y = rand::random_range(0..height);
        let b_x = rand::random_range(0..width);
        let b_y = rand::random_range(0..height);
        Line::new(Point::new(a_x, a_y), Point::new(b_x, b_y))
    }

    fn get_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let dx = self.b.x - self.a.x;
        let dy = self.b.y - self.a.y;
        let quantity = dx.abs().min(dy.abs());
        for i in 0..=quantity {
            let t = (i as f32) / (quantity as f32);
            let x_pnt = (self.a.x as f32) + (dx as f32) * t;
            let y_pnt = (self.a.y as f32) + (dy as f32) * t;
            points.push(Point::new(x_pnt.round() as i32, y_pnt.round() as i32));
        }
        points
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let points = &self.get_points();
        for point in points {
            image.display(point.x, point.y, self.color.clone());
        }
    }
}

#[derive(Debug)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self { a: a.clone(), b: b.clone(), c: c.clone() }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = Triangle::color();
        let lines = Vec::from([
            Line::new(self.a.clone(), self.b.clone()),
            Line::new(self.b.clone(), self.c.clone()),
            Line::new(self.c.clone(), self.a.clone()),
        ]);

        for mut line in lines {
            line.color = color.clone();
            line.draw(image);
        }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self { a: a.clone(), b: b.clone() }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = Rectangle::color();
        let lines = Vec::from([
            Line::new(self.a.clone(), Point::new(self.b.clone().x, self.a.clone().y)),
            Line::new(Point::new(self.b.clone().x, self.a.clone().y), self.b.clone()),
            Line::new(self.b.clone(), Point::new(self.a.clone().x, self.b.clone().y)),
            Line::new(Point::new(self.a.clone().x, self.b.clone().y), self.a.clone()),
        ]);

        for mut line in lines {
            line.color = color.clone();
            line.draw(image);
        }
    }
}
//circle section
#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self { center: Point::new(center.x, center.y), radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let radius = rand::random_range(5..width);
        Circle::new(&center, radius)
    }
}

//  x = cx + r * cos(θ) and y = cy + r * sin(θ)
// teta will be starting from 0 to 2PI
impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let perimter = 2.0 * PI * (self.radius as f64);
        let steps = (2.0 * PI) / (perimter as f64);
        let mut i: f64 = 0.0;
        let color = Circle::color();
        while i <= 2.0*PI {
            let x = (self.center.x as f64) + (self.radius as f64) * i.cos();
            let y = (self.center.y as f64) + (self.radius as f64) * i.sin();
            image.display(x.round() as i32, y.round() as i32, color.clone());
            i += steps;
        }
    }
}
