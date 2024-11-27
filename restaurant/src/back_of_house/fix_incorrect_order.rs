cook_order(); // This function is directly accessable because it is defined in the same scope/
super::deliver_order(); // super keyword is used for getting the parent module to be available in this scope.
// crate::deliver_order();