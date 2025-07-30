use rand::Rng;
use raster::Color;
use raster::Image;

pub trait Drawable {
    fn draw(&self, _:&mut Image);
    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color::rgb(rng.random(),rng.random(),rng.random())
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color) ;
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn random(width: i32, height: i32) -> Self{
        let mut rng = rand::rng();
        let x = rng.random_range(0..=width);
        let y = rng.random_range(0..=height);
        Self::new(x, y)
    }
    pub fn new(x: i32, y:i32) -> Self {
        Self {
            x: x,
            y: y,
        }
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
       img.display(self.x, self.y, self.color());
    }
}


#[derive(Debug)]
pub struct Line (Point, Point);

impl Line {
    pub fn random(width: i32, height: i32) -> Self{
        Self(Point::random(width, height), Point::random(width, height))
    }

    pub fn new(a: Point, b: Point) -> Self {
        Self(a, b)
    }

    pub fn draw_with_color(&self, img: &mut Image, color: Color) {
        let Line(p0, p1) = self;

        let mut x = p0.x as f32;
        let mut y = p0.y as f32;

        let dx = (p1.x - p0.x) as f32;
        let dy = (p1.y - p0.y) as f32;

        let steps = dx.abs().max(dy.abs());

        let x_inc = dx / steps;
        let y_inc = dy / steps;

        for _ in 0..=steps as i32 {
            img.display(x.round() as i32, y.round() as i32, color.clone());
            x += x_inc;
            y += y_inc;
        }
    }
}

impl Drawable for Line {
   fn draw(&self, img: &mut Image) {
        let color = self.color();
        self.draw_with_color(img, color)
    }
}

#[derive(Debug)]
pub struct Triangle (Point, Point, Point);

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self(*a, *b, *c)
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        let color = self.color();
        let l1 = Line::new(self.0,self.1);
        let l2 = Line::new(self.1,self.2);
        let l3 = Line::new(self.2,self.0);
        l1.draw_with_color(img, color.clone());
        l2.draw_with_color(img, color.clone());
        l3.draw_with_color(img, color.clone());
    }
}


#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let edge = Point::random(width, height);
        let radius = ((edge.x - center.x).pow(2) + (edge.y - center.y).pow(2)) as f32;
        Self::new(center, radius.sqrt() as i32)
    }
    pub fn new(a: Point, b: i32) -> Self {
        Self {
            center: a,
            radius: b,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, img : &mut Image){

        let color = self.color();
        let mut x = 0;
        let cx = self.center.x ;
        let cy = self.center.y ;
        let mut y = - self.radius;
        let mut p = - self.radius;
        while x < -y {
            if p > 0 {
                y += 1;
                p += 2*(x+y) +1
            }else{
                p+= 2*x + 1
            }
            img.display(cx + x , cy +y , color.clone());
            img.display(cx - x , cy +y , color.clone());
            img.display(cx + x , cy -y , color.clone());
            img.display(cx - x , cy -y , color.clone());

            img.display(cx + y , cy +x , color.clone());
            img.display(cx + y , cy -x , color.clone());
            img.display(cx - y , cy +x , color.clone());
            img.display(cx - y , cy -x , color.clone());

            x += 1;

        }
    }
}

#[derive(Debug)]
pub struct Rectangle (Point, Point, Point, Point);

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let c = Point::new(a.x, b.y);
        let d = Point::new(b.x, a.y);
        Self(*a, *b, c, d)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img : &mut Image){
        let color = self.color();
        let l1 = Line::new(self.0,self.2);
        let l2 = Line::new(self.2,self.1);
        let l3 = Line::new(self.1,self.3);
        let l4 = Line::new(self.3,self.0);
        l1.draw_with_color(img, color.clone());
        l2.draw_with_color(img, color.clone());
        l3.draw_with_color(img, color.clone());
        l4.draw_with_color(img, color.clone());

    }
}