use indoc::indoc;
use insta::assert_snapshot;
use sway_fmt_v2::{config::user_def::FieldAlignment, Formatter};

use std::sync::Arc;

fn format_with(formatter: &mut Formatter, src: &str) -> String {
    formatter.format(Arc::from(src), None).unwrap()
}

fn format(src: &str) -> String {
    let mut formatter = Formatter::default();
    format_with(&mut formatter, src)
}

#[test]
fn test_const() {
    assert_snapshot!(format(indoc! {r"
    contract;
    pub const TEST:u16=10;
    "}), @r###"
    contract;
    pub const TEST: u16 = 10;
    "###);
}

#[test]
fn test_struct_alignment() {
    let mut formatter = Formatter::default();
    formatter.config.structures.field_alignment = FieldAlignment::AlignFields(40);

    assert_snapshot!(format_with(&mut formatter, indoc! {r"
    contract;
    pub struct Foo<T, P> {
    barbazfoo: u64,
    baz  : bool,
    }
    "}), @r###"
    contract;
    pub struct Foo<T, P> {
        barbazfoo : u64,
        baz       : bool,
    }
    "###)
}

#[test]
fn test_struct() {
    assert_snapshot!(format(indoc! {r"
    contract;
    pub struct Foo {
        bar: u64,
        baz: bool
    }
    "}), @r###"
    contract;
    pub struct Foo {
        bar: u64,
        baz: bool,
    }
    "###)
}

#[test]
fn test_enum_without_variant_alignment() {
    assert_snapshot!(format(indoc! {r"
    contract;

    enum Color {
        Blue: (), Green: (),
                Red: (),
        Silver: (),
                        Grey: () }
    "}), @r###"
    contract;

    enum Color {
        Blue: (),
        Green: (),
        Red: (),
        Silver: (),
        Grey: (),
    }
    "###)
}

#[test]
fn test_enum_with_variant_alignment() {
    // Creating a config with enum_variant_align_threshold that exceeds longest variant length
    let mut formatter = Formatter::default();
    formatter.config.structures.field_alignment = FieldAlignment::AlignFields(20);

    assert_snapshot!(format_with(&mut formatter, indoc! {r"
    contract;

    enum Color {
        Blue: (), Green: (),
                Red: (),
        Silver: (),
                        Grey: (), }
    "}), @r###"
    contract;

    enum Color {
        Blue   : (),
        Green  : (),
        Red    : (),
        Silver : (),
        Grey   : (),
    }
    "###)
}

#[test]
fn test_item_abi_with_generics_and_attributes() {
    assert_snapshot!(format(indoc! {r"
    contract;

    abi StorageMapExample {
        #[storage(write)]fn insert_into_map1(key: u64, value: u64);

    fn hello(key: u64, value: u64);
    }"}), @r###"
    contract;

    abi StorageMapExample {
        #[storage(write)]
        fn insert_into_map1(key: u64, value: u64);

        fn hello(key: u64, value: u64);
    }
    "###)
}

#[test]
fn test_multi_items() {
    assert_snapshot!(format(indoc! {r"
    contract;

    pub const TEST: u16 = 10;
    pub const TEST1: u16 = 10;"}), @r###"
    contract;

    pub const TEST: u16 = 10;
    pub const TEST1: u16 = 10;
    "###)
}

#[test]
fn test_ty_formatting() {
    assert_snapshot!(format(indoc! {r"
    contract;

    enum TestTy {
        Infer:
        _,
        Array : [u8;
        40],
        String:         str[
        4
        ],
        PathType     : root::
    example::
        type,
        TupleNil: (),
        Tuple: (   u64,
            u32
        ),
    }"}), @r###"
    contract;

    enum TestTy {
        Infer: _,
        Array: [u8; 40],
        String: str[4],
        PathType: root::example::type,
        TupleNil: (),
        Tuple: (u64, u32),
    }
    "###);
}

#[test]
fn test_storage_without_alignment() {
    assert_snapshot!(format(indoc! {r"
    contract;
    struct Type1 {
        foo: u64,
    }

    struct Type2 {
        bar: u64,
    }

    storage {
        var1: Type1=Type1{ foo: 8 },
            var2: Type2=Type2{ bar: 9 },
    }
    "}), @r###"
    contract;
    struct Type1 {
        foo: u64,
    }

    struct Type2 {
        bar: u64,
    }

    storage {
        var1: Type1 = Type1 { foo: 8 },
        var2: Type2 = Type2 { bar: 9 },
    }
    "###)
}

#[test]
fn test_storage_with_alignment() {
    let mut formatter = Formatter::default();
    formatter.config.structures.field_alignment = FieldAlignment::AlignFields(50);

    assert_snapshot!(format_with(&mut formatter, indoc! {r"
    contract;
    struct Type1 {
        foo: u64,
    }

    struct Type2 {
        bar: u64,
    }

    storage {
    long_var_name: Type1=Type1{ foo: 8 },
        var2: Type2=Type2{ bar: 9 },
    }
    "}), @r###"
    contract;
    struct Type1 {
        foo : u64,
    }

    struct Type2 {
        bar : u64,
    }

    storage {
        long_var_name : Type1 = Type1 { foo: 8 },
        var2          : Type2 = Type2 { bar: 9 },
    }
    "###)
}

#[test]
fn test_storage_initializer() {
    assert_snapshot!(format(indoc! {r"
    contract;

    struct Type1 {
        x: u64,
        y: u64,
    }

    struct Type2 {
        w: b256,
        z: bool,
    }

    storage {
        var1: Type1 = Type1 {



            x: 0,

            y:
            0,
            },
        var2: Type2 = Type2 { w: 0x0000000000000000000000000000000000000000000000000000000000000000,z: false,
        },
    }"}), @r###"
    contract;

    struct Type1 {
        x: u64,
        y: u64,
    }

    struct Type2 {
        w: b256,
        z: bool,
    }

    storage {
        var1: Type1 = Type1 { x: 0, y: 0 },
        var2: Type2 = Type2 {
            w: 0x0000000000000000000000000000000000000000000000000000000000000000,
            z: false,
        },
    }
    "###)
}

#[test]
fn test_item_fn() {
    assert_snapshot!(format(indoc! {r"
    contract;

    pub fn hello( person: String ) -> String {let greeting = 42;greeting.to_string()}
    fn goodbye() -> usize {let farewell: usize = 5; farewell }"}), @r###"
    contract;

    pub fn hello(person: String) -> String {
        let greeting = 42;
        greeting.to_string()
    }
    fn goodbye() -> usize {
        let farewell: usize = 5;
        farewell
    }
    "###)
}

#[test]
fn test_same_line_where() {
    assert_snapshot!(format(indoc! {r"
    contract;

    pub fn hello( person: String ) -> String where T: Eq,{let greeting = 42;greeting.to_string()}"}), @r###"
    contract;

    pub fn hello(person: String) -> String
    where
        T: Eq,
    {
        let greeting = 42;
        greeting.to_string()
    }
    "###)
}

#[test]
fn test_trait_and_super_trait() {
    assert_snapshot!(format(indoc! {r"
    library traits;

    trait Person{ fn name( self )->String;fn age( self )->usize; }
    trait Student:Person {fn university(self) -> String;}
    trait Programmer {fn fav_language(self) -> String;}
    trait CompSciStudent: Programmer+Student {fn git_username(self) -> String;}"}), @r###"
    library traits;

    trait Person {
        fn name(self) -> String;

        fn age(self) -> usize;
    }
    trait Student: Person {
        fn university(self) -> String;
    }
    trait Programmer {
        fn fav_language(self) -> String;
    }
    trait CompSciStudent: Programmer + Student {
        fn git_username(self) -> String;
    }
    "###)
}

#[test]
fn test_method_calls() {
    let mut formatter = Formatter::default();
    formatter.config.structures.small_structures_single_line = true;
    formatter.config.whitespace.max_width = 220;

    assert_snapshot!(format_with(&mut formatter, indoc! {r"
    script;

    struct Opts {
        gas: u64,
        coins: u64,
        id: ContractId,
    }

    fn  main(       ) -> bool{
        let default_gas  = 1_000_000_000_000           ;let fuelcoin_id = ~ContractId::from(0x018f59fe434b323a5054e7bb41de983f4926a3c5d3e4e1f9f33b5f0f0e611889);

        let balance_test_id = ~ContractId :: from( 0x597e5ddb1a6bec92a96a73e4f0bc6f6e3e7b21f5e03e1c812cd63cffac480463 ) ;

        let fuel_coin = abi(    TestFuelCoin, fuelcoin_id.into(       ) ) ;

        assert(fuelcoin_balance == 0);

        fuel_coin.mint        {
            gas:             default_gas
        }

        (11);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        assert( fuelcoin_balance   == 11 ) ;

        fuel_coin.burn {
            gas: default_gas
        }
        (7);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        assert(fuelcoin_balance == 4);

        fuel_coin.force_transfer {
            gas: default_gas
        }
        (3, fuelcoin_id, balance_test_id);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        let balance_test_contract_balance = balance_of(fuelcoin_id, balance_test_id);
        assert(fuelcoin_balance == 1);
        assert(balance_test_contract_balance == 3);

        true
    }"}), @r###"
    script;

    struct Opts {
        gas: u64,
        coins: u64,
        id: ContractId,
    }

    fn main() -> bool {
        let default_gas = 1_000_000_000_000;
        let fuelcoin_id = ~ContractId::from(0x018f59fe434b323a5054e7bb41de983f4926a3c5d3e4e1f9f33b5f0f0e611889);

        let balance_test_id = ~ContractId::from(0x597e5ddb1a6bec92a96a73e4f0bc6f6e3e7b21f5e03e1c812cd63cffac480463);

        let fuel_coin = abi(TestFuelCoin, fuelcoin_id.into());

        assert(fuelcoin_balance == 0);

        fuel_coin.mint { gas: default_gas }(11);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        assert(fuelcoin_balance == 11);

        fuel_coin.burn { gas: default_gas }(7);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        assert(fuelcoin_balance == 4);

        fuel_coin.force_transfer { gas: default_gas }(3, fuelcoin_id, balance_test_id);

        fuelcoin_balance = balance_of(fuelcoin_id, fuelcoin_id);
        let balance_test_contract_balance = balance_of(fuelcoin_id, balance_test_id);
        assert(fuelcoin_balance == 1);
        assert(balance_test_contract_balance == 3);

        true
    }
    "###)
}

#[test]
fn test_struct_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    // This is a comment, for this one to be placed correctly we need to have Module visitor implemented
    pub struct Foo { // Here is a comment



        // Trying some ASCII art
        baz:u64,




        bazzz:u64//  ________ ___  ___  _______   ___               ___       ________  ________  ________
                // |\  _____\\  \|\  \|\  ___ \ |\  \             |\  \     |\   __  \|\   __  \|\   ____\
                // \ \  \__/\ \  \\\  \ \   __/|\ \  \            \ \  \    \ \  \|\  \ \  \|\ /\ \  \___|_
                //  \ \   __\\ \  \\\  \ \  \_|/_\ \  \            \ \  \    \ \   __  \ \   __  \ \_____  \
                //   \ \  \_| \ \  \\\  \ \  \_|\ \ \  \____        \ \  \____\ \  \ \  \ \  \|\  \|____|\  \
                //    \ \__\   \ \_______\ \_______\ \_______\       \ \_______\ \__\ \__\ \_______\____\_\  \
                //     \|__|    \|_______|\|_______|\|_______|        \|_______|\|__|\|__|\|_______|\_________\
                //                                                                                  \|_________|
    }
    // This is a comment
    "}), @r###"
    contract;
    // This is a comment, for this one to be placed correctly we need to have Module visitor implemented
    pub struct Foo { // Here is a comment



        // Trying some ASCII art
        baz: u64,
        bazzz: u64,//  ________ ___  ___  _______   ___               ___       ________  ________  ________
                // |\  _____\\  \|\  \|\  ___ \ |\  \             |\  \     |\   __  \|\   __  \|\   ____\
                // \ \  \__/\ \  \\\  \ \   __/|\ \  \            \ \  \    \ \  \|\  \ \  \|\ /\ \  \___|_
                //  \ \   __\\ \  \\\  \ \  \_|/_\ \  \            \ \  \    \ \   __  \ \   __  \ \_____  \
                //   \ \  \_| \ \  \\\  \ \  \_|\ \ \  \____        \ \  \____\ \  \ \  \ \  \|\  \|____|\  \
                //    \ \__\   \ \_______\ \_______\ \_______\       \ \_______\ \__\ \__\ \_______\____\_\  \
                //     \|__|    \|_______|\|_______|\|_______|        \|_______|\|__|\|__|\|_______|\_________\
                //                                                                                  \|_________|
    }
    // This is a comment
    "###)
}

#[test]
fn test_enum_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    pub enum Bazz { // Here is a comment
        // Trying some ASCII art
        baz: (),





        bazzz: (),//-----
                //--D--
                //-----
    }
    "}), @r###"
    contract;
    pub enum Bazz { // Here is a comment
        // Trying some ASCII art
        baz: (),
        bazzz: (),//-----
                //--D--
                //-----
    }
    "###);
}

#[test]
fn test_fn_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    // This is a comment before a fn
    // This is another comment before a fn
    fn hello_world( baz: /* this is a comment */ u64) { let x = 5; // This is a comment inside the block
    }
    "}), @r###"
    contract;
    // This is a comment before a fn
    // This is another comment before a fn
    fn hello_world(baz: /* this is a comment */ u64) {
        let x = 5; // This is a comment inside the block
    }
    "###);
}

#[test]
fn test_abi_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    // This is an abi
    abi StorageMapExample {
        // insert_into_map is blah blah
        #[storage(write)] // this is some other comment
        fn insert_into_map(key: u64, value: u64);
        // this is the last comment inside the StorageMapExample
    }"}), @r###"
    contract;
    // This is an abi
    abi StorageMapExample {
        // insert_into_map is blah blah
        #[storage(write)] // this is some other comment
        fn insert_into_map(key: u64, value: u64);
        // this is the last comment inside the StorageMapExample
    }
    "###);
}

#[test]
fn test_const_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    pub const /* TEST: blah blah tests */ TEST: u16 = 10; // This is a comment next to a const"}), @r###"
    contract;
    pub const /* TEST: blah blah tests */ TEST: u16 = 10; // This is a comment next to a const
    "###);
}
#[test]
fn test_storage_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;

    struct Type1 {
        foo: u64,
    }
    struct Type2 {
        bar: u64,
    }
    storage {
        // Testing a comment inside storage
        long_var_name: Type1=Type1{ foo: 8},
        // Testing another comment
        var2: Type2 = Type2{bar:9} // This is the last comment
    }"}), @r###"
    contract;

    struct Type1 {
        foo: u64,
    }
    struct Type2 {
        bar: u64,
    }
    storage {
        // Testing a comment inside storage
        long_var_name: Type1 = Type1 { foo: 8 },
        // Testing another comment
        var2: Type2 = Type2 { bar: 9 }, // This is the last comment
    }
    "###);
}

#[test]
fn test_trait_comments() {
    assert_snapshot!(format(indoc! {r"
    contract;
    // This is the programmer trait
    trait Programmer {
        // Returns fav languages of this Programmer.
        fn fav_language(self) -> String;
    }"}), @r###"
    contract;
    // This is the programmer trait
    trait Programmer {
        // Returns fav languages of this Programmer.
        fn fav_language(self) -> String;
    }
    "###)
}

#[test]
fn test_where_comment() {
    assert_snapshot!(format(indoc! {r"
    contract;

    pub fn hello( person: String ) -> String where /* This is next to where */ T: Eq, /*Here is a comment*/{let greeting = 42;greeting.to_string()}"}), @r###"
    contract;

    pub fn hello(person: String) -> String
    where /* This is next to where */
        T: Eq, /*Here is a comment*/
    {
        let greeting = 42;
        greeting.to_string()
    }
    "###)
}

#[test]
fn test_impl() {
    assert_snapshot!(format(indoc! {r"
    script;

    struct Foo {
        bar: u64,
        baz: bool,
    }

    trait Qux {
        fn is_baz_true(self) -> bool;
    }

    impl<A ,     B>    Qux<A, B> for
    Foo
    where
        A    : Qux,
        B: Qux    ,
    {fn is_baz_true(self) -> bool {
            self.baz
        }}"}), @r###"
    script;

    struct Foo {
        bar: u64,
        baz: bool,
    }

    trait Qux {
        fn is_baz_true(self) -> bool;
    }

    impl<A, B> Qux<A, B> for Foo where
        A: Qux,
        B: Qux,
    {
        fn is_baz_true(self) -> bool {
            self.baz
        }
    }
    "###)
}

#[test]
fn test_impl_without_generics() {
    assert_snapshot!(format(indoc! {r"
    script;

    struct Foo {
        bar: u64,
        baz: bool,
    }

    trait Qux {
        fn is_baz_true(self) -> bool;
    }

    impl   Qux for 
    Foo 
    {fn is_baz_true(self) -> bool {
            self.baz
        }}"}), @r###"
    script;

    struct Foo {
        bar: u64,
        baz: bool,
    }

    trait Qux {
        fn is_baz_true(self) -> bool;
    }

    impl Qux for Foo {
        fn is_baz_true(self) -> bool {
            self.baz
        }
    }
    "###)
}

#[test]
fn test_newline_sequence_formatting() {
    assert_snapshot!(format(indoc! {r"
    script;

    fn main() {
        let number: u64 = 10;

        let number2: u64 = 20;



        let number3: u64 = 30;



    }"}), @r###"
    script;

    fn main() {
        let number: u64 = 10;

        let number2: u64 = 20;

        let number3: u64 = 30;
    }
    "###)
}
