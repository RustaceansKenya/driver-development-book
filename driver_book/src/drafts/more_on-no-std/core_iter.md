# core::iter


Turn any data-structure into something that you can iterate through. ie access things is a sequential and unit-wise manner.  
You can turn a collection into an iterable-object ie turn it into an iterator.  

To turn a normal data-structure into an iterable data-structure, you have to implement the `Iterator trait` for that struct. ie implement the functions found under the `Iterator` trait... give it some extra super-powers.  

The core of Iterator looks like this:

```rust
trait Iterator {
    type Item;  // The unit data structure. This is the unit of division. One unit == one step in our iteration process
    fn next(&mut self) -> Option<Self::Item>;  // this fetches the next unit of division
}
```


When you define the `iterator` trait, the `IntoIterator` gets defined automatically.  
```rust
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
```  

Pre-defined Collections implement both the `Iterator` and `IntoIterator` trait. They have additional functions that are not found in custom structs.  
These extra functions are :  

- iter() - creates an iterator that uses `&Ts` of the collection as its Iterator units.  
- iter_mut() - creates an iterator that uses `&mut Ts` of the collection as its iterator units.  
- into_iter() // this function can be found for all iterators, it is not specific to Collections. It consumes the caller by value. It is useful when chaining Iterator-operations.


The for loop and the match statement use the `into_iter` function by default. So they consume the Iterator or collections that get passed to them.  