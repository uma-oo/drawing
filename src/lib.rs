use rand::prelude::*;


pub mod geometrical_shapes {

   pub trait Drawable {
       fn draw(&mut self);
       fn color(&self);
   }


   pub trait Displayable {
    fn display(&mut self, x:i32, y:i32, color:Color);
   }

    
   // struct of point
    pub struct Point {
    x : i32,
    y : i32,

   }



   // implement it 

   impl  Point { 
    fn new(x: i32, y:i32) -> Self {
     Self {
       x:x,
       y:y
      }
    }

   fn random( width: i32, height:i32)-> Self{
    let y = rand::;
    let x = rand::thread_rng().gen_range(1..=width);
    Point::new(x,y)
   }
   }

   pub struct Line {
    a : Point,
    b : Point,
   }


    impl  Line {

    }

   pub struct Triangle {
    a : Point ,
    b : Point,
    c : Point,
   }

   impl Triangle {

   }

   pub struct Rectangle {
    a : Point,
    b : Point,
    c : Point,
    d : Point,
   }

   impl Rectangle {

   }

   pub struct  Circle {
    center : Point,
    radius : i32,
   }

   impl  Circle {
    
    fn new(point: Point , radius:i32) ->Self {
        Self{
            center:point,
            radius:radius,
        }
    }

    fn random(width:i32 , height : i32) ->Self{
       let x_point = rand::thread_rng().gen_range(1..width);
       let y_point = rand::thread_rng().gen_range(1..height);
       // generate the raduis but how random length 
       Circle::new(x_point, y_point)
    }


   }
 
   // we need some random numbers form 0..255 





}
