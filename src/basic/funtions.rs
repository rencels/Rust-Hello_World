//Name, Parameters, Return type, body - Named function
//1 Named functions
pub fn anatomy(a:i32, b:i32) -> i32 {

    if a > 10 {
        return 666; 
    }

    a + b
}

//2. Annonymous functions(closures) -> do not have a nice, special syntax and can capture outside environment
pub fn test_annonymous(a:i32, b:i32) -> i32 {

    let my_num = 42; //we can capture this value from the closuse

    let my_anon_fn = |a: i32, b: i32| if a > my_num { a + b } else { a - b };

    my_anon_fn(a , b)    
}





//3 Methods on the instances of the type(struct) - Function Associated With a Type
#[derive(Debug)]
pub struct Point {
  pub X: i32,
  pub Y: i32    
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point
    {
        Point {
            X: x,
            Y: y
         }
    }

    pub fn print(&self)
    {
        println!("X:{} Y:{}", self.X, self.Y);
    }
    
    pub fn add(&mut self, new_value: i32)
    {
        self.X += new_value;
        self.Y += new_value;
    }
}

//Associated funtions -> similar to static methods You do not need an instance