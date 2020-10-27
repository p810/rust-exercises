pub mod my_library {
    pub mod greeting {
        pub fn say_hello_to(subject: &String) {
            super::display::speak(&format!("Hello, {}!", subject));
        }

        pub fn say_hey_to(subject: &String) {
            super::display::speak(&format!("Hey, {}!", subject));
        }
    }

    pub mod display {
        // 
        // this function could be referenced using either an absolute or relative path, like so respectively:
        // 
        //   - crate::my_library::display::speak(...)
        //   - my_library::display::speak(...)
        // 
        // the relative path would need to occur from somewhere in this directory, if I'm not mistaken
        // 
        pub fn speak(greeting: &String) {
            println!("{}", greeting);
        }
    }

    // 
    // similar to a class in a language like PHP or TypeScript, the properties and
    // methods of a struct must be marked as public if they're going to be referenced
    // by third party callers
    // 
    pub struct Breakfast {
        pub oatmeal: Oatmeal,
        fruit: Fruit,
    }

    impl Breakfast {
        pub fn oatmeal_with_apple_slices(oatmeal: Oatmeal) -> Breakfast {
            Breakfast {
                oatmeal,
                fruit: Fruit::Apple,
            }
        }

        pub fn oatmeal_with_orange_slices(oatmeal: Oatmeal) -> Breakfast {
            Breakfast {
                oatmeal,
                fruit: Fruit::Orange,
            }
        }
    }

    pub fn what_is_for_breakfast(breakfast: Breakfast) -> String {
        let oatmeal = match breakfast.oatmeal {
            Oatmeal::Regular => "Plain oatmeal",
            Oatmeal::AppleCinnamon => "Apple cinnamon oatmeal",
            Oatmeal::Peach => "Peach oatmeal",
        };

        let fruit = match breakfast.fruit {
            Fruit::Apple => "with a side of apple slices",
            Fruit::Orange => "with a side of orange slices",
            Fruit::Peach => "with a peach on the side",
            Fruit::Blueberry => "with a side of blueberries",
        };

        format!("{} {}", oatmeal, fruit)
    }

    //
    // unlike structs, all of the variants of an enum are considered public if the
    // enum is marked as such
    //
    pub enum Oatmeal {
        Regular,
        AppleCinnamon,
        Peach,
    }

    enum Fruit {
        Apple,
        Orange,
        Peach,
        Blueberry,
    }
}
