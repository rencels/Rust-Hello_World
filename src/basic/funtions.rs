//Name, Parameters, Return type, body - Named function
pub fn anatomy(a:i32, b:i32) -> i32 {

    if a > 10 {
        return 666; 
    }

    a + b
}

pub fn test_annonymous(a:i32, b:i32) -> i32 {

    let my_anon_fn = |a: i32, b: i32| if a > b { a + b } else { a - b };

    my_anon_fn(a , b)    
}

//Named functions

//Annonymous functions(closures) -> do not have a nice, special syntax and can capture outside environment

//Methods on the instances of the type(struct)

//Associated funtions -> similar to static methods You do not need an instance