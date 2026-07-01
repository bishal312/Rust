/*
Iterators in Rust provide a powerful and flexible way to process data efficientyl
by transforming, filtering, and aggregating elements in a collection.

However, unlike traditional loops, Rust's iterators are lazy, meaning they don't do any work until explicitly instructed to .
often combining multiple transformations into a single pass over the data.

In this lesson , we,'' dive into the core of Rust's iterator system,
exploring how to use methods like .map(), .filter(), and .fold() to create expressive, functional code.
*/

fn main() {
    // Iterators - example 1
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for value in v1_iter {
        println!("Got: {}",value);
    }

    // Iterator demonstration - example 2
    let v1: Vec<i32> = vec![10, 100, 1000];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&100));
    assert_eq!(v1_iter.next(), Some(&1000));
    assert_eq!(v1_iter.next(), None);

    // Types of iterators

    /*
    1. .iter()
    purpose: Creates an iterator that borrows each element in the collection immutably.
    Ownership: The iterator yields references (&T), not owned values: the original collection is accessible and unaltered.
    Use Case: Use iter() when you want to read or inspect elements without taking ownership or modifying the collection.
    
    */

    // eg:-
    let numbers: Vec<i32> = vec![1, 3, 5];

    for value in numbers.iter() {
        println!("Got:- {}", value);
    }
    
    println!("iter()");
    println!("{:?}", numbers);
    /*
    2. iter_mut()
    Purpose: Creates an iterator that borrows each element in a collection mutably.
    Ownership: THe iterator yields mutable references (&mut T), allowing you to modify the elements in place.
    Use Case: Use iter_mut when you need to change the elements of a collection.
    */
    // eg2:-
    let mut numbers: Vec<i32> = vec![11, 22, 33];

    for num in numbers.iter_mut() {
        *num += 1;
        println!("Got:- {}", num);
    }
    println!("iter_mut()");
    println!("{:?}", numbers);

    /*
    3. into_iter()
    Purpose: Consumes the collection and creates an iterator that takes ownership of each element.
    Ownership: The iterator yields owned values (T). After iteration, the original collecion is no longer accessible.
    Use Case: Useful when you want to transfer ownership of the elements. 
     */
    // eg3:-
    let numbers: Vec<i32> = vec![1, 2, 3];

    for num in numbers.into_iter() {
        println!("Got: {}", num);
    }
    // println!("{:?}", numbers); here i cannot print numbers collection because of into_iter's ownership consuming property.


    /*
        Summary Table:

        | Iterator      | Yields   | Ownership | Purpose |
        | :---          | :---     | :---      | :---    |
        | `iter()`      | `&T`     | Borrow    | Read-only iteration |
        | `iter_mut()`  | `&mut T` | Mutable   | Allows in-place modification of elements |
        | `into_iter()` | `T`      | Ownership | Consumes collection for ownership |

        They give fine-grained control over data ownership and mutability during iteration.
    */


    // Methods to modify or consume iterators: map(), filter(), fold()

    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // Map
    let squares: Vec<_> = numbers
        .iter()
        .map(|&x| x * x)
        .collect();

    println!("Map - Squares {:?}", squares);

    // Filter
    let even: Vec<_> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("Filter - Even {:?}", even);

    // Fold (similar to reduce)
    let fold: i32 = numbers
        .iter()
        .fold(0, |acc, &x| acc + x);

    println!("Fold - Sum {}", fold);


}

pub trait Iterator {
    type Item; //associated type - Item
    fn next(&mut self) -> Option<Self::Item>;
}