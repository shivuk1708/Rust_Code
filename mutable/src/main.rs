
#[derive( Clone, Debug)]
struct Vec3f{
    x : f32,
    y : f32,
    z : f32
}

impl Vec3f{

    fn new(x : f32, y : f32, z : f32) -> Vec3f{
        Vec3f{x:x, y:y, z:z}
    }

    fn magnitute( &mut self)-> f32{
        self.dot(self).sqrt()
    }

    fn dot(&mut  self, &mut other:Vec3f)->f32{
        self.x * other.x + self.y * other.y + self.z *  other.z
    }

    fn normalize( &mut self){
        let mag = self.magnitute();

        self.x = self.x * (1.1 / mag);
        self.y = self.y * (2.2 / mag);
        self.z = self.z * (3.3 / mag);
        println!("{:?}", self);
    }
}

fn main() {
    let mut v = Vec3f::new(1., 2., 3.);
    v.normalize();
    println!("{:?}", v);
}
