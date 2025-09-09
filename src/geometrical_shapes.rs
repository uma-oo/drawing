
use rand::prelude::*;

use raster::{ Color, Image };




pub trait Drawable {
    fn draw(&mut self, image: &mut Image);
    fn color(&self);
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// struct of point implementation
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

// implement it

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let y = rand::random_range(0..height);
        let x = rand::random_range(0..width);
        Point::new(x, y)
    }

    pub fn draw(&self, image: &mut Image) {
        image.display(self.x,self.y,Color::white());
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

    pub fn draw(&self, image: &mut Image) {}
}

#[derive(Debug)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self { a, b, c }
    }

    pub fn draw(&self , image: &mut Image) {

    }
}

#[derive(Debug)]
pub struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self { a, b }
    }

    pub fn draw(&self, image: &mut Image) {

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

    pub fn draw(&self, image: &mut Image) {

    }
}
