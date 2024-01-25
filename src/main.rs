fn main() {
   struct Car {
       color: String,
       speed: u32
   }
   impl Car {
       fn driving(&self)  {
           println!("{} car is driving", self.color);
       }
   }
   let car = Car {
    color: String::from("Black"),
    speed: 100,
   };
   let car2 = Car {
       color: String::from("Red"), 
       speed:53,
   };

   let mut count = 0;
   let cars = [&car, &car2];
    
   car2.driving();

   for car in cars.iter() {
        count = count + 1;
        println!("count = {}, color = {}, speed = {}", count, car.color, car.speed);
   }
}


