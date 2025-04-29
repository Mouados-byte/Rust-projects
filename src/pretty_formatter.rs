trait Format{
    fn standard(&self);
    fn pretty(&self);
    fn print_both_formats(&self) {
        println!("Standard formatting");
        self.standard();
        println!("Pretty formatting");
        self.pretty();
        println!("");
    }
}

struct Person {
    first_name: String,
    last_name: String,
    email: String,
    age: i32
}

struct Product {
    name: String,
    price: f64,
    in_stock: bool
}

impl Product{
    fn stock(&self) -> String {
        let return_value = if self.in_stock {"In Stock"} else {"Out of Stock"};
        return_value.to_string()
    }
}


impl Format for String {
    fn standard(&self) {        
        println!("{}", self);
    }
    fn pretty(&self) {
        println!("\"{}\"", self);
    }
}

impl Format for i32{
    fn standard(&self) {        
        println!("{}", self.to_string());
    }
    fn pretty(&self) {
        let mut placeholder: Vec<String> = Vec::new();
        let mut current_int = self.clone();
        let mut modulo = current_int % 1000;
        loop {
            placeholder.insert(0, modulo.to_string());
            if modulo == current_int{
                break
            }

            current_int -= modulo;
            current_int /= 1000;
            modulo = current_int % 1000
        }
        println!("{}", placeholder.join(","));
    }
}

impl Format for Person{
    fn standard(&self) {
        println!("{} {}, ({}) <{}>", self.last_name, self.first_name, self.age, self.email);
    }

    fn pretty(&self) {
        println!("Person:");
        println!("  Full Name: {} {}", self.last_name, self.first_name);
        println!("  Age: ({})", self.age);
        println!("  Email: <{}>", self.email);
    }
}

impl Format for Product{
    fn standard(&self) {
        println!("{} - {}$ [{}]", self.name, self.price, if self.in_stock {"In Stock"} else {"Out of Stock"});
    }

    fn pretty(&self) {
        println!("Pretty: {} - {}$ [{}]", self.name, self.price, self.stock());
    }
}

fn main(){
    let string = "This is a string".to_string();
    let integer = 1232193873;
    let person = Person {first_name: "Mouad".to_string(), last_name: "Imam".to_string(), age: 22, email: "imam6mouad@gmail.com".to_string()};
    let product = Product {name: "Nectary".to_string(), price: 0.55, in_stock: false};

    // Standard Output
    string.standard();
    integer.standard();
    person.standard();
    product.standard();
    
    println!("");
    // Pretty Output
    string.pretty();
    integer.pretty();
    person.pretty();
    product.pretty();

    println!("");
    string.print_both_formats();
    integer.print_both_formats();
    person.print_both_formats();
    product.print_both_formats();
}