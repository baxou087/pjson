
// use crate::data::TypeVal;


/// Struct that will contain the json type Array
/// An array is a vector of TypeVal since it can contain different value types
pub struct Array {
    array: Vec<TypeVal>
}


impl Array {
    /// Array constructor that only initialises the array
    pub fn new() -> Array {
        Array {
            array: Vec::new()
        }
    }


    /// Method for adding a value in the array
    pub fn add_value(&mut self, val: TypeVal) {
        self.array.push(val);
    }


}