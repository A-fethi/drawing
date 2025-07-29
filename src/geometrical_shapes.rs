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

#[derive(Debug, Clone)]
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
        let mut rng = rand::rng();
        let color = Color::rgb(rng.random(),rng.random(),rng.random());
        self.draw_with_color(img, color)
    }
}

#[derive(Debug)]
pub struct Triangle (Point, Point, Point);

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self(a.clone(), b.clone(), c.clone())
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        let mut rng = rand::rng();
        let color = Color::rgb(rng.random(),rng.random(),rng.random());
        let l1 = Line::new(self.0.clone(),self.1.clone());
        let l2 = Line::new(self.1.clone(),self.2.clone());
        let l3 = Line::new(self.2.clone(),self.0.clone());
        l1.draw_with_color(img, color.clone());
        l2.draw_with_color(img, color.clone());
        l3.draw_with_color(img, color.clone());
    }
}


// #[derive(Debug)]
// pub struct Circle (Point, Point);

// impl Circle {
//     pub fn random(width: i32, height: i32) -> Self {
//          Self(Point::random(width, height), Point::random(width, height))
//     }
//     pub fn new(a: Point, b: Point) -> Self {
//         Self(a, b)
//     }
// }

#[derive(Debug)]
pub struct Rectangle (Point, Point, Point, Point);

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        let c = Point::new(a.x, b.y);
        let d = Point::new(b.x, a.y);
        Self(a.clone(), b.clone(), c, d)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img : &mut Image){
         let mut rng = rand::rng();
        let color = Color::rgb(rng.random(),rng.random(),rng.random());
        let l1 = Line::new(self.0.clone(),self.2.clone());
        let l2 = Line::new(self.2.clone(),self.1.clone());
        let l3 = Line::new(self.1.clone(),self.3.clone());
        let l4 = Line::new(self.3.clone(),self.0.clone());
        l1.draw_with_color(img, color.clone());
        l2.draw_with_color(img, color.clone());
        l3.draw_with_color(img, color.clone());
        l4.draw_with_color(img, color.clone());

    }
}