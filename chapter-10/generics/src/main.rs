fn main() {
    println!("Generics function");

    let list1: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    let list2: Vec<f64> = vec![1.9, 2.87, 3.34, 44.54, 53.8, 6.5];

    // let largest = largest(&list1);
    let largest2 = largest(&list2);
    println!("The largest num is {:?}", largest2);

    // let point1: Point<i32> = Point {x: 12, y: 14}; // we have to use same type number in both variables
    // let point2: Point<f64> = Point {x: 32.3, y:43.3};

    let point: Point = Point::new(23.3, 44.4);

    

    // To use different type in two different variables, we can use two Generic types like Point<T, U>
}


// Duplication of functions because of types!!!

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut result: &i32 = &list[0];

//     for item in list {
//         if item > result {
//             result = item;
//         }
//     }
//     *result
// }

// fn largest_f64(list: &[f64]) -> f64 {
//     let mut result: &f64 = &list[0];

//     for item in list {
//         if item > result {
//             result = item;
//         }
//     }
//     *result
// }

// Now we will use Generic types in single function.

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result: &T = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }
    result
}

// Struct Point<T> {
//     x: T,
//     y: T,
// }

Struct Point<T, U> {
    x: T,
    y: U,
}

imp<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self {x, y}
    }
}