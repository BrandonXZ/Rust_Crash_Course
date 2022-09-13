use egui::{Id, util::IdTypeMap};

/**
 * Egui has a weird syntax and implementation for saving widget state so this is broken down here...
 * 
 */

pub fn run() {

    let a = Id::new("a");
    let b = Id::new("b");
    let mut map: IdTypeMap = Default::default();

    // `a` associated with an f64 and an i32
    map.insert_persisted(a, 3.14);
    map.insert_temp(a, 42);

    // `b` associated with an f64 and a `&'static str`
    map.insert_persisted(b, 13.37);
    map.insert_temp(b, "Hello World".to_owned());

    //Now lets get those values **Note the ::<> "turbofish syntax" which is basically a simple way of casting to a type without exclusively annotating for that type 
    let value_1 = map.get_temp::<f64>(a); 
    let value_2 = map.get_temp::<i32>(a);

    //notice that we can pull 2 values with the exact same id of a or b, this is because the ID is taking the data type into consideration...
    let value_3 = map.get_temp::<f64>(b);
    let value_4 = map.get_temp::<String>(b);

    //Now lets print those values

    println!("\nThis is the first A Value: {:?}\n", value_1); // I find it rather odd that we have to print these with the debug fmt trait since the type is already known. 
    println!("\nThis is the second A Value: {:?}\n", value_2); 

    println!("\nThis is the first B Value: {:?}\n", value_3);
    println!("\nThis is the second B Value: {:?}\n", value_4);// and normally doesn't require debug <---This is just a string 

    //Now lets try it in a loop
    let (check, map) = loop_persist();
    if check {
        let retrieved_info = getFromLoop(map);
        println!("\nFinal retrieved Data: {:#?}\n", retrieved_info );
    } else {
        println!("\n\nuhh ohh something went wrong...");
    }


}

pub fn loop_persist()  -> (bool, IdTypeMap) {
    let mut map: IdTypeMap = Default::default();

    for i in 0..10 {
        let id = Id::new(i);
        let mut saved_val = i + 10;
        println!("ID: {:#?} is getting the value: {}", id.clone(), saved_val.clone());
        map.insert_temp(id,  saved_val.clone());
        
    }
    // println!("Map is: {:#?}", map.clone()); // No need to print this out, its not in the recognizable hex format and value is a hash of what was input....

    (true, map)
}


pub fn getFromLoop(mut map: IdTypeMap) -> Vec<i32> {
    let mut result_vec = vec![];

    for i in 0..10 {
        let id = Id::new(i); //definitely shouldn't do this ever but instead of passing the Id's we're assuming what they are based on the previous loop
        let values = map.get_temp::<i32>(id);
        println!("During loop ID: {:#?} pulled Value: {:#?}", id.clone(), values.clone());

        result_vec.push(values.unwrap().clone());
    }

    result_vec
}