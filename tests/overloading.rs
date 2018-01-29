#[macro_use]
extern crate adapton;

mod overloading_preserves_results {
    
    use adapton::macros::*;
    use adapton::engine::*;
    
    #[test]
    fn basic_ops() {
        let a1 = cell!(123);
        let a2 = cell!(456);
        let a3 = a1 + a2;
        let a3n = -a3;
        
        assert_eq!(get!(a3n), -579);
        
        let b1 = cell!(true);
        let b2 = cell!(false);
        let b2n = !b2;
        
        assert_eq!(get!(b1 & b2n), true);
        
        let a4 = cell!(1) + cell!(2);
        
        assert_eq!(a4, cell!(1) + cell!(2));
        assert_ne!(a4, cell!(1) + cell!(5));
        assert_ne!(a4, cell!(1) - cell!(2));
    }
    
}
