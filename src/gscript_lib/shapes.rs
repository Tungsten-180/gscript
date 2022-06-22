use std::f64::consts::PI;

#[derive(Debug,Clone,Copy)]
pub struct Rectangle {
    pub c1: [f64; 2],
    pub c2: [f64; 2],
}

impl Rectangle{
    pub fn corners(&self)->[[f64;2];4]{
        [self.c1,[self.c1[0],self.c2[1]],self.c2,[self.c2[0],self.c1[1]],]
    }
    pub fn difference(&self)->[f64;2]{
        let [t,u]=self.c1;
        let [e,i]=self.c2;
        [t-e,u-i]
    }
    pub fn mean_difference(&self)->f64{
        let [x,y]=self.difference();
        ((x+y)/2.0).abs()
    }
    pub fn min_difference(&self)->f64{
        let [x,y] = self.difference();
        match x.abs() > y.abs() {
            true=>y.abs(),
            false=>x.abs(),
        }
    }
}

impl Shape for Rectangle {
    fn valid(&self) -> bool {
        let [x, y] = self.c1;
        let [z, s] = self.c2;
        (x.abs() <= 100.0) && (y.abs() <= 100.0) && (z.abs() <= 100.0) && (s.abs() <= 100.0)
    }
    fn area(&self) -> f64 {
        let [x, y] = self.c1;
        let [z, _s] = self.c2;
        { (x - z) * (x - y) }.abs()
    }
    fn sideln(&self, side: usize) -> f64 {
        let index = match side {
            0 => [0, 0],
            2 => [0, 0],
            1 => [1, 1],
            3 => [1, 1],
            _ => [5, 5],
        };
        { self.c1[index[0]] - self.c2[index[1]] }.abs()
    }
    fn slope(&self, point: [usize; 2]) -> [f64; 2] {
        [self.sideln(point[0]), self.sideln(point[1])]
    }
}

pub struct Triangle {
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
}

impl Shape for Triangle {
    fn valid(&self) -> bool {
        let [x, y] = self.p1;
        let [z, s] = self.p2;
        let [l, k] = self.p0;
        (self.p0 != self.p1)
            && (self.p0 != self.p2)
            && (self.p1 != self.p2)
            && (x.abs() <= 100.0)
            && (y.abs() <= 100.0)
            && (z.abs() <= 100.0)
            && (s.abs() <= 100.0)
            && (l.abs() <= 100.0)
            && (k.abs() <= 100.0)
    }
    fn slope(&self, point: [usize; 2]) -> [f64; 2] {
        [
            match point[0] {
                0 => self.p0,
                1 => self.p1,
                2 => self.p2,
                _ => panic!(),
            }[0] - match point[1] {
                0 => self.p0,
                1 => self.p1,
                2 => self.p2,
                _ => panic!(),
            }[0],
            match point[0] {
                0 => self.p0,
                1 => self.p1,
                2 => self.p2,
                _ => panic!(),
            }[1] - match point[1] {
                0 => self.p0,
                1 => self.p1,
                2 => self.p2,
                _ => panic!(),
            }[1],
        ]
    }
    fn sideln(&self, side: usize) -> f64 {
        let [x, y] = match side {
            0 => self.slope([1, 2]),
            1 => self.slope([0, 1]),
            2 => self.slope([0, 2]),
            _ => panic!(),
        };
        let [x, y] = [x, y].map(|x| x * x);
        let l = { x + y }.sqrt();
        l
    }
    fn area(&self) -> f64 {
        let [a, b, c] = [self.sideln(0), self.sideln(1), self.sideln(2)];
        let s = { a + b + c } / 2.0;

        { s * (s - a) * (s - b) * (s - c) }.sqrt()
    }
}

pub struct Circle {
    pub o: [f64; 2],
    pub r: f64,
}

impl Shape for Circle {
    fn valid(&self) -> bool {
        let [x, y, r] = [self.o[0], self.o[1], self.r];
        (x + r <= 100.0) && (y + r <= 100.0)
    }
    fn area(&self) -> f64 {
        (self.r*self.r)*PI
    }
    fn sideln(&self, side: usize) -> f64 {
        match side{
            0=>{},
            _=>panic!(),
        };
        2.0*PI*self.r
    }
    fn slope(&self, _point: [usize; 2]) -> [f64; 2] {
        [0.0;2]
    }
}

impl Circle{
    pub fn steps(&self)->Vec<[f64;2]>{
        let mut steps:Vec<[f64;2]> = Vec::new();
        let dr = 1.0/(2.0*self.r);
        let mut n = 1.0;
        let maxn = 4.0*PI*self.r;
        while n <= maxn{
            steps.push([(dr*n).cos()*self.r,(dr*n).sin()*self.r]);
            n+=1.0;
        }
        steps
    }
}

pub trait Shape {
    fn valid(&self) -> bool;
    fn area(&self) -> f64;
    fn sideln(&self, side: usize) -> f64;
    fn slope(&self, point: [usize; 2]) -> [f64; 2];
}
