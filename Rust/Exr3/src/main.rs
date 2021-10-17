mod client;
mod network;
mod product;

//cloneable structure
#[derive(Clone,Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Clone,Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
        }
    }
}

//Imenuvani kortezhi
struct Point(f32, f32, f32);

fn main() {
    let user = User {
        username: String::from("Иванчо"),
        email: String::from("ivan40@abv.bg"),
        sign_in_count: 10,
    };
    let user_ref: &User = &user;
    let mut user2 = User {
        username: String::from("Goshko"),
        email: String::from("jori99@abv.bg"),
        sign_in_count: 11,
    };
    user2.email = "jori.jorjev@abv.bg".to_string();
    println!("{}, {}", user2.username, user2.email);
    user2 = user.clone();
    println!("{}, {}", user2.username, user2.email);
    println!("user1 = {:?}", user);

    //Update syntax - it's moving the fields
    let hacker: User = User {
        email: String::from("hackerManVani@hac.com"),
        ..user
    };
    println!("user1 = {:?}", hacker);

    //Short syntax for creating a structure
    let mut width = 2.0;
    let mut height = 3.0;
    let mut rect = Rectangle {
        width,   // <-
        height,  // <-
    };
    height = 3.45; //does not change the struct
    rect.width = 4.0; // this changes it
    println!("rect = {:?}", rect);

    let user = User::new(String::from("Иванчо"),
                         String::from("ivan40@abv.bg"));
    println!("Area={}", rect.area());

    //Imenuvani kortezhi
    let mut p = Point(0.0, 0.0, 0.0);
    p.0 = 0.1;

    //using modules
    crate::client::connect();

    let user = product::new(
        String::from("Иванчо"),
        String::from("ivan40@abv.bg"),
    );
    println!("ProductUser={}", user.get_username());
}
