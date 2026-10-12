use std::ops::Add;

struct Person {
    first_name: String,
    last_name: String,
}

struct Marriage {
    husband: Person,
    wife: Person,
    location: String,
    date: chrono::NaiveDate,
}

impl Add for Person {
    type Output = Marriage;
    fn add(self, rhs: Self) -> Self::Output {
        let new_marriage = Marriage{
            husband: self, 
            wife: rhs, 
            location: "Arizon".to_string(), 
            date: chrono::offset::Local::now().date_naive(),
        };
        return new_marriage;
    }
}

struct GroceryItem {
    name: String,
    price: f32,
}

struct GroceryBill {
    items: Vec<GroceryItem>,
    tax_rate: f32
}

impl Add<GroceryItem> for GroceryBill {
    type Output = GroceryBill;
    fn add(self, rhs: GroceryItem) -> Self::Output {
        let mut bill = self;

        bill.items.push(rhs);
        return bill;
    }
}

impl GroceryBill{
    fn calculate_total(&self) -> f32 {
        let items_total = self.items.iter().fold(0f32, |a, i| return a + i.price);
        let tax_value = items_total * self.tax_rate;
        return items_total + tax_value;
    }
}

fn main() {
    //Marriage example
    let person1 = Person{first_name: "Trevor".to_string(), last_name: "Suliivan".to_string()};
    let person2 = Person{first_name: "Nancy".to_string(), last_name: "Jones".to_string()};

    let marriage = person1 + person2;
    println!("{} got married to {} on the date {}", marriage.husband.first_name, marriage.wife.first_name, marriage.date);

    //Grocery example
    let mut new_bill = GroceryBill{items: Vec::new(), tax_rate: 0.027};

    let carrots = GroceryItem{name: "Bag of Carrots of 1kg".to_string(), price: 2.2};
    let cheese = GroceryItem{name: "Chottage cheese".to_string(), price: 23.4};

    new_bill = new_bill + carrots + cheese; // First new_bill + carrots executes and then new_bill is updated and then new_bill + cheese is called
    let total = new_bill.calculate_total();

    println!("Total of your bill is {total}");
}