// test = random(a, b, c) % 255
// color = Color::rba(a, b, c);

use rand::prelude::*;

use raster::{ Color, Image };

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self)-> Color{
         Color::white()
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// struct of point implementation
#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

// implement it
// done with it


impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let y = rand::random_range(0..height);
        let x = rand::random_range(0..width);
        Point::new(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

#[derive(Debug)]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            a: a,
            b: b,
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
        let quantity = dx.abs() + dy.abs() ;
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
            point.draw(image);
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
        Line::new(self.a.clone(), self.b.clone()).draw(image);
        Line::new(self.b.clone(), self.c.clone()).draw(image);
        Line::new(self.c.clone(), self.a.clone()).draw(image);
        // there is smtg wrong with this
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
        Line::new(self.a.clone(), Point::new(self.b.clone().x, self.a.clone().y)).draw(image);
        Line::new(Point::new(self.b.clone().x, self.a.clone().y), self.b.clone()).draw(image);
        Line::new(self.b.clone(), Point::new(self.a.clone().x, self.b.clone().y)).draw(image);
        Line::new(Point::new(self.a.clone().x, self.b.clone().y), self.a.clone()).draw(image);
    }
}

// circle section
#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn new(point: Point, radius: i32) -> Self {
        Self {
            center: point,
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let x_point = rand::random_range(0..width);
        let y_point = rand::random_range(0..height);
        let radius = rand::random_range(0..=width.min(height));
        Circle::new(Point::new(x_point, y_point), radius)
    }
}



impl Drawable for Circle {
    fn color(&self)-> Color {
        let r = rand::random_range(0..=255);
        let g = rand::random_range(0..=255);
        let b = rand::random_range(0..=255);
        Color::Rgb(r,g,b)
    }
    fn draw(&self, image: &mut Image) {
        let teta = 0;


    }
}
