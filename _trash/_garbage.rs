use std::str;
use std::collections::HashMap;
use std::{cmp::Eq, hash::Hash};
use std::fmt::Display;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fs;



struct Cacher<U, T> where T: FnMut(U) -> U{
    closure: T,
    map: HashMap<U, U>,
    result: Option<U>,
}

impl<U, T> Cacher<U, T> where T: FnMut(U) -> U, U: Eq + Hash + Display + Copy{
    fn new(_: U, closure: T) -> Cacher<U, T>{
        Cacher{
            closure,
            map: HashMap::new(),
            result: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.result{
            Some(v) => v,
            None => {
                let result = self.map.entry(arg).or_insert((self.closure)(arg));
                self.result = Some(*result);
                *result
            }
        }
    }
}


fn generate_workout(intensity: u32, random_number: u32) {
    let a_simple_var: u8 = 34;
	let callback = move |num: u32| -> u32 {
            a_simple_var = 56;
            println!("a simple var just moved here");
            println!("calculating slowly...");
            num+1 // we can add one to the num because this closure can mutate its environment vairable and it moves them to its scope!
        
      };
      
    let mut expensive_result = Cacher::new(34, callback);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", expensive_result.value(intensity)
            );
        }
    }
}


fn trash(){
	
	   {
            'outer: loop{
                println!("this is the outer loop");
                'inner: loop{
                    println!("this is the inner loop");
                    // break; // only the inner loop

                    break 'outer;
                }

                println!("this print will never be reached");
            }




            let mut counter = 0;
            let result = loop{
                counter += 1;
                if counter == 10{
                break counter * 2;
                }
            };
            println!("counter is {}", counter);
            println!("result is {}", result);

	    }
	

        // ------------------------------ testing trait Copy and Clone for closure ------------------------------
        let outside_num = 353;
            let callback = move |num: i32| {
                let got_outside_num = outside_num;
                let copy_of_num = num; //-- trait Copy is implemented for i32 thus has trait Clone so we don't need to clone it and we can also access it after it's moved into copy_of_num var 
            };

        // ------------------------------ testing trait Copy and Clone for i32 and String/str ------------------------------
        let name = String::from("wildonion");
        let name_slice = &name[0..3]; // pointing to an offset on the heap by borrowing some parts of the name String
        let anot_cop_of_slice = name_slice; // this is ok cause the Copy trait is implemented for &T which is &str in here
        // NOTE - we have still access to name_slice in here
        // ...
        // this is no ok cause name is on the heap with a saved reference to the heap on the stack also it doesn't implement Copy trait
        // the Clone trait is implemented for that because of double free pointer issue at runtime and the implementation of drop trait.
        // let another_name = name;
        // println!("name is droped {:?}", name); 
        let another_name = name.clone(); // we used the clone method here to copy the whole the reference on the stack and the whole data on the heap as well 
        let another_name = &name; // this is ok cause the Copy trait is implemented for &T which in our case is &String which is coerced &str or string slice which is saved somewhere in the memory(heap, stack or binary)
        let number: i32 = 3534;
        let another_number = number; // this is ok cause the number it's on the stack thus the drop trait is not implemented for that(still got the number even it's moved) so we can copy the whole number variable into another_number

        // ------------------------------ testing trait Copy and Clone for u8 and Vec<u8> ------------------------------
        // u8 implements Copy
        let x: u8 = 123;
        let y = x;
        // x can still be used
        println!("x={}, y={}", x, y);

        // Vec<u8> implements Clone, but not Copy
        let v: Vec<u8> = vec![1, 2, 3];
        let w = v.clone();
        //let w = v // This would *move* the value, rendering v unusable.

        // ------------------------------ testing trait Copy and Clone for structs ------------------------------
        #[derive(Debug, Clone, Copy)]
        pub struct PointCloneAndCopy {
            pub x: f64,
        }

        #[derive(Debug, Clone)]
        pub struct PointCloneOnly {
            pub x: f64,
        }

        fn test_copy_and_clone() {
            let p1 = PointCloneAndCopy { x: 0. };
            let p2 = p1; // because type has `Copy`, it gets copied automatically.
            println!("{:?} {:?}", p1, p2);
        }

        fn test_clone_only() {
            let p1 = PointCloneOnly { x: 0. };
            let p2 = p1; // because type has no `Copy`, this is a move instead. to avoid moving we can clone the p1
            println!("{:?} {:?}", p1, p2);
        }

	

        // reading image pixels or bytes
        // ...
        if let Ok(bytes) = fs::read("/home/wildonion/Pictures/test.jpg"){
            println!("image bytes >>>> {:?}", bytes);
        }

	

        'outer: for x in 0..5 {
            'inner: for y in 0..5 {
                println!("{},{}", x, y);
                if y == 3 {
                    break 'outer;
                }
            }
        }


    // 	::::::::::iterator for struct::::::::::
	struct Alternate {
	    state: i32,
	}

	impl Iterator for Alternate {
	    type Item = i32;

	    fn next(&mut self) -> Option<i32> {
            let val = self.state;
            self.state = self.state + 1;

            // if it's even, Some(i32), else None
            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
	    }
	}

	let mut iter = Alternate { state: 0 };

	// we can see our iterator going back and forth
	assert_eq!(iter.next(), Some(0));
	assert_eq!(iter.next(), None);
	assert_eq!(iter.next(), Some(2));
	assert_eq!(iter.next(), None);




    // =============================================================================================================================
    
    /*
	
        let mut my_name = "Pascal".to_string();
        my_name.push_str( " Precht");
        let last_name = &my_name[7..];
        
        
        
                         buffer
                        /    capacity
                       /    /   length
                      /    /   /
                    +–––+–––+–––+
        stack frame │ • │ 8 │ 6 │ <- my_name: String
                    +–│–+–––+–––+
                      │
                    [–│–––––––– capacity –––––––––––]
                      │
                    +–V–+–––+–––+–––+–––+–––+–––+–––+
                heap │ P │ a │ s │ c │ a │ l │   │   │
                    +–––+–––+–––+–––+–––+–––+–––+–––+

                    [––––––– length ––––––––]
                    
                    
                    
        Notice that last_name does not store capacity information on the stack. 
        This is because it’s just a reference to a slice of another String that manages its capacity. 
        The string slice, or str itself, is what’s considered ”unsized”. 
        Also, in practice string slices are always references so their type will always be &str instead of str.
                    
                    

                    my_name: String   last_name: &str
                    [––––––––––––]    [–––––––]
                    +–––+––––+––––+–––+–––+–––+
        stack frame │ • │ 16 │ 13 │   │ • │ 6 │ 
                    +–│–+––––+––––+–––+–│–+–––+
                      │                 │
                      │                 +–––––––––+
                      │                           │
                      │                           │
                      │                         [–│––––––– str –––––––––]
                    +–V–+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+–––+
                heap │ P │ a │ s │ c │ a │ l │   │ P │ r │ e │ c │ h │ t │   │   │   │
                    +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
                    
                    

        string literals are a bit special. They are string slices that refer to “preallocated text” 
        that is stored in read-only memory as part of the executable. In other words, 
        it’s memory that ships with our program and doesn’t rely on buffers allocated in the heap.
        that said, there’s still an entry on the stack that points to that preallocated memory when the program is executed:

        
        let my_name = "Pascal Precht";
        
        
                    my_name: &str
                    [–––––––––––]
                      +–––+–––+
        stack frame   │ • │ 6 │ 
                      +–│–+–––+
                        │                 
                        +––+                
                            │
            preallocated  +–V–+–––+–––+–––+–––+–––+
            read-only     │ P │ a │ s │ c │ a │ l │
            memory        +–––+–––+–––+–––+–––+–––+
        
        
                    
			    
	*/
	let first_name = "Pascal"; // str - &str is a reference to String some where in the heap
    let last_name = "Precht".to_string(); // turn to String
    let another_last_name = String::from("Precht");
    greet(first_name); // first_name is &str by default
    greet(&last_name); // last_name is passed by reference
    greet(&another_last_name); // another_last_name is passed by reference

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

        
    let name = String::from("erfan"); // String
    let another_name = "another erfan"; // str
    let combined = name + &another_name;
    // name.push_str(&another_name); // name moved due to above operator
    println!("{}", combined);
    // println!("{}", name); // error - borrowed after move
    println!("{}", another_name);

    let sample_string = String::from("wildonion");
    let bytes = sample_string.bytes(); // turn a string into buffer (asccii)
    println!("[..] two first bytes of the string are : {}", &sample_string[0..2]); // byte indices
    println!("[..] the string bytes : {:?}", bytes);

    let text = "hello hello from wildonion here double again again wildonion";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0); // return a mutable reference inserted or the old value
        *count += 1; // updating the old value by dereferencing it, cause count is a mutable reference of the value 
    }

    println!("{:?}", map);

    // =============================================================================================================================
    
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
	

    
	
	
	
	
	
	
	
	
	
	// =============================================================================================================================
	// NOTE - generic types in function signature can be bounded to lifetimes and traits so we can use the lifetime to avoid having dangling pointer of the generic type in function body and traits to extend the type interface

	impl<'a, Pack: Interface + 'a> Into<Vec<u8>> for Unpack<'a, Pack, SIZE>{ //-- based on orphan rule we have to import the trait inside where the struct is or bound the instance of the struct into the Into trait in function calls - we wanto to return the T inside the wrapper thus we can implement the Into trait for the wrapper struct which will return the T from the wrapper field
	    fn into(self) -> Vec<u8> {
		self.arr.to_vec()
	    }
	}


	pub const SIZE: usize = 325;
	pub type Context<'a, Pack> = Unpack<'a, Pack, SIZE>; //-- Pack type will be bounded to Interface trait and 'l lifetime 
	pub struct Unpack<'l, T: Interface + 'l + Into<T>, const U: usize>{ //-- T is of type Pack struct which is bounded to 'l lifetime the Into and the Interface traits and U (constant generic) must be a constant usize type - Unpack takes a generic type of any kind which will be bounded to a trait and a lifetime but it must be referred to a field or be inside a PhantomData since T and the lifetime will be unused and reserved by no variables inside the ram
	    pub pack: T, //-- pack is a pointer or a reference and is pointing to T which is a generic type and bounded to a trait and a valid lifetime as long as the lifetime of the struct instance
	    pub arr: &'l [u8; U],
	}

	pub struct Pack; //-- we've allocated some space inside the stack for this struct when defining it which has long enough lifetime to initiate an instance from it using struct declaration and return a reference to that instance inside any function 
	pub trait Interface{}

	impl Interface for Pack{} //-- is required for return_box_trait() function

	fn return_none_trait<T>() -> () where T: Interface{ // NOTE - `T` type must be bound to Interface trait

	}

	fn return_impl_trait() -> impl Interface { // NOTE - returning impl Trait from function means we're implementing the trait for the object that is returning from the function regardless of its type that we're returning from the function cause compiler will detect the correct type in compile time and implement or bound the trait for that type
	    Pack {}
	}

	fn return_box_trait() -> Box<dyn Interface + 'static> { // NOTE - returning Box<dyn Trait> from function means we're returning a struct inside the Box which the trait has implemented for and since traits have unknown size at compile time we must put them inside the Box with a valid lifetime like 'static
	    Box::new(Pack {})
	}

	impl Pack{ ////// RETURN BY POINTER EXAMPLE //////


	    fn new() -> Self{


		let hello = "Здравствуйте";
		let s = &hello[0..2];
		// every index is the place of an element inside the ram which has 1 byte size which is taken by that element
		// in our case the first element takes 2 bytes thus the index 0 won't return 3 
		// cause place 0 and 1 inside the ram each takes 1 byte and the size of the
		// first element is two bytes thus &hello[0..2] which is index 0 and 1 both returns 3 
		// and we can't have string indices in rust due to this reason!


		///////////////////////////////////////////// ENUM MATCH TEST
		#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
		enum Chie{
		    Avali(u8),
		    Dovomi(String),
		    Sevomi,
		}


		let ine = Chie::Avali(12); //-- the Dovomi variant is never constructed cause we've used the first variant  

		match ine{
		    Chie::Avali(value) if value == 23 => { //-- matching on the Avali arm if the value was only 23
			println!("u8 eeee");

		    },
		    Chie::Dovomi(value) if value == "wildonion".to_string() => { //-- matching on the Dovomi arm if the value was only "wildonion" string
			println!("stringeeee");
		    },
		    _ => {
			println!("none of them");
		    }
		}

		// --------------- CODEC OPS ON ENUM ---------------
		let encoded = serde_json::to_vec(&Chie::Sevomi); ////// it'll print a vector of utf8 encoded JSON
		let decoded = serde_json::from_slice::<Chie>(&encoded.as_ref().unwrap()); //-- as_ref() returns a reference to the original type

		let encoded_borsh = Chie::Sevomi.try_to_vec().unwrap(); ////// it'll print 2 cause this the third offset in memory
		let decoded_borsh = Chie::try_from_slice(&encoded_borsh).unwrap();

		/////////////////////////////////////////////
		Pack{}
	    }

	    fn ref_struct(num_thread: &u8) -> &Pack{ //-- returning ref from function to a pre allocated data type (not inside the function) Pack struct in our case, is ok
		let instance = Pack::new(); //-- since new() method of the Pack struct will return a new instance of the struct which is allocated on the stack and is owned by the function thus we can't return a reference to it or as a borrowed type
		// &t //-- it's not ok to return a reference to `instance` since `instance` is a local variable which is owned by the current function and its lifetime is valid as long as the function is inside the stack and executing which means after executing the function its lifetime will be dropped
		let instance = &Pack{}; //-- since we're allocating nothing on the stack inside this function thus by creating the instance directly using the the Pack struct and without calling the new() method which is already lives in memory with long enough lifetime we can return a reference to the location of the instance of the pack from the function
		instance //-- it's ok to return a reference to `instance` since the instance does not allocate anything on the stack thus taking a reference to already allocated memory with long enough lifetime is ok since the allocated memory is happened in struct definition line
	    }

	    // NOTE - argument can also be &mut u8
	    pub fn ref_str_other_pointer_lifetime(status: &u8) -> &str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we can use the lifetime of the passed in argument, the status in this case which has been passed in by reference from the caller and have a valid lifetime which is generated from the caller scope by the compiler to return the pointer from the function
		let name = "wildonion";
		name //-- name has a lifetime as valid as the passed in status argument lifetime from the caller scope 

	    }

	    // NOTE - first param can also be &mut self; a mutable reference to the instance and its fields
	    pub fn ref_to_str_other_self_lifetime(&self) -> &str{ //-- in this case we're good to return the pointer from the function or send a copy to the caller's space since we can use the lifetime of the first param which is &self which is a borrowed type of the instance and its fields (since we don't want to lose the lifetime of the created instance from the contract struct after calling each method) and have a valid lifetime (as long as the instance of the type is valid) which is generated from the caller scope by the compiler to return the pointer from the function
		let name = "wildonion";
		name //-- name has a lifetime as valid as the first param lifetime which is a borrowed type of the instance itself and its fields and will borrow the instance when we want to call the instance methods
	    }

	    // NOTE - 'a lifetime has generated from the caller scope by the compiler
	    pub fn ref_to_str_specific_lifetime<'a>(status: u8) -> &'a str{ //-- in this case we're good to return the pointer from the function or copy to the caller's space since we've defined a valid lifetime for the pointer of the return type to return the pointer from the function which &'a str
		let name = "wildonion";
		name //-- name has a lifetime as valid as the generated lifetime from the caller scope by the compiler and will be valid as long as the caller scope is valid
	    }

	    // NOTE - 'static lifetime will be valid as long as the whole lifetime of the caller scope (it can be the main function which depends on the whole lifetime of the app)
	    pub fn ref_to_str_static() -> &'static str{
		let name = "wildonion";
		name //-- name has static lifetime valid as long as the whol lifetime of the caller scope which can be the main function which will be valid as long as the main or the app is valid
	    }
		
	    //// ERROR - can't return a reference to heap allocated data structure from function due to their unknown size at compile time and they are temprary value
	    // pub fn ref_to_string<'s>() -> &'s String{
	    //     let name = &"wildonion".to_string();
	    //     name //-- ERROR - we can't return this or &"wildonion".to_string() since they are temporary value due to the fact that heap data structure's size are not specific at compile time and they are some kina a temporary value thus heap data structures can't be returned in their borrowed form from the function since their size are not specific at compile time therefore by taking a pointer to the location of them we might have dangling pointer later once their location gets dropped during the function lifetime body 
	    // }

	    pub fn ref_to_num<'n>() -> &'n i32{
		let num = 23;
		// &num //-- ERROR - we can't return this since the num is owned by the current function and returning the reference to the local variable which is owned by the function is denied
		&23 //-- we can return &23 since we did allocate nothing on the stack inside the function (which this can be done by creating a local variable inside the function) and we're just returning a pointer to the location of a number directly   

	    }

	}
	
	
	
	trait Some{}
	impl Some for Boxed{}

	type Boxed = Box<dyn FnMut() + 'static>; //-- we must bound the type that wants to be a pointer or to be a referenced from a heap location like FnMut() closure to a valid lifetime like 'static
	let var: Boxed = Box::new(||{});


	fn call<'a>(a: &'a Boxed) -> () where Boxed: Some + 'a{ //-- in order to bind the Boxed to Some trait the Some trait must be implemented for the Boxed - can't bound a lifetime to a self-contained type means we can't have a: u32 + 'static
	     // 'a lifetime might be shorter than static
	     //...    
	}
	

	
	


}
