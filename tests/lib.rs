#![no_std]

use surjective_enum::From;

#[test]
fn test() {
    const OOF : u16 = 0b10;
    #[repr(u8)]
    #[derive(From,PartialEq,Debug)]
    pub enum Enu {
        /// This is Bar
        Bar = 0b00,
        /// and this is foo
        Foo = 0b01,
        Oof = OOF as u8,
        Rab = 0b11,
    }
    assert_eq!( Enu::from( 0b10), Enu::Oof); 
    assert_eq!( Enu::from( 0b101), Enu::Rab);     

    #[derive(From,PartialEq,Debug)]
    #[repr(u8)]
    pub enum Foo {
        One,
        Two = 2,
        Three,
        Four = 5
    }
    
    assert_eq!( Foo::from( Foo::Three as u8), Foo::Three);
}
