[![crates.io](https://img.shields.io/crates/v/add_getters_setters.svg)](https://crates.io/crates/add_getters_setters)

# add_getters_setters

Makes it much easier to add getters and setters for fields of structures.

Done by simply just adding the appropriate meta tag(s) onto the struct or fields of the struct
which will generate the corresponding methods for you.

# usage

- Add this crate into your Cargo.toml file

- Add the following line to the top of `main.rs`

	`#[macro_use] extern crate add_getters_setters;`

- Add the `#[derive()]` tag on your struct, and derive the necessary functions:
	- `AddGetter` gives you access to the `#[get]` tag which will generate a function which returns a reference to the field(s) the tag is on.
	- `AddGetterMut` gives you access to the `#[get_mut]` tag which will generate a function which returns a mutable reference to the field(s) the tag is on.
	- `AddGetterVal` gives you access to the `#[get_val]` tag which will generate a function which returns a copy of the fields data (only available since version 1.1.0).
	- `AddSetter` gives you access to the `#[set]` tag which will generate a function that you can use to set the value of the field.

- Add the appropriate meta tags to each field to generate the methods, or since version 1.0.0, you can put the tags onto the struct itself which will generate the corresponding methods on every field of the struct (see example at the bottom of this file).

| Function to derive | Tag | Signature of generated method |
|--|--|--|
| AddGetter | #[get] | `pub fn get_{field name}(&self) -> &{field data type}` |
| AddGetterVal | #[get_val] | `pub fn {field name}(&self) -> {field data type}` |
| AddGetterMut | #[get_mut] | `pub fn get_{field name}_mut(&mut self) -> &mut {field data type}` |
| AddSetter | #[set] | `pub fn set_{field name}(&mut self, v: {field data type})` |

Note that all generated functions are public methods.

# Example

    struct HorseRider {
	    //stuff here
	}

    #[derive(AddGetter, AddGetterVal, AddGetterMut, AddSetter)]
    struct RaceHorse {
	    #[get]
	    name: String,
	    
	    #[get]
	    #[get_val]
	    #[set]
	    speed: i16,

	    #[get]
	    #[get_mut]
	    rider: HorseRider,
	}
With this code, these methods would be generared for you...

    impl RaceHorse {
	    pub fn get_name(&self) -> &String {
		    &self.name
	    }
	    
	    pub fn get_speed(&self) -> &i16 {
		    &self.speed
	    }

		pub fn speed(&self) -> i16 {
		    self.speed // implicit copy here
	    }
	    
	    pub fn set_speed(&mut self, v: i16) {
		    self.speed = v;
	    }
	    
	    pub fn get_rider(&self) -> &HorseRider {
		    &self.rider
	    }
	    
	    pub fn get_rider_mut(&mut self) -> &mut HorseRider {
		    &mut self.rider
	    }
	}
	    
# Since version 1.0.0

Add a getter and a setter for every field by adding the `#[get]` and `#[set]` tags to the struct definition:

    #[derive(AddGetter, AddSetter)]
    #[get]
    #[set]
    struct Dragon {
	    name: String,
	    age: u64,
	    weight: u32
	}
which has the same effect as doing the following:

    #[derive(AddGetter, AddSetter)]
    struct Dragon {
	    #[get]
	    #[set]
	    name: String,
	    
	    #[get]
	    #[set]
	    age: u64,
		
	    #[get]
	    #[set]
	    weight: u32
	}