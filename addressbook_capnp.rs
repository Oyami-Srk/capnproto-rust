pub mod Person {
    use layout::*;
//    use addressbook_capnp::*;

    pub static STRUCT_SIZE : StructSize = StructSize {data : 1, pointers : 4,
                                                      preferredListEncoding : INLINE_COMPOSITE};

    list_submodule!(addressbook_capnp, Person)

    pub struct Reader<'self> {
        _reader : StructReader<'self>
    }

    impl <'self> Reader<'self> {

        pub fn new<'a>(reader : StructReader<'a>) -> Reader<'a> {
            Reader{ _reader : reader }
        }

        pub fn totalSizeInWords(&self) -> uint {
            self._reader.totalSize() as uint
        }
    }

    pub struct Builder {
        _builder : StructBuilder
    }

    impl Builder {
        pub fn new(builder : StructBuilder) -> Builder {
            Builder { _builder : builder }
        }

        pub fn setId(&self, value : u32) {
            self._builder.setDataField::<u32>(0, value);
        }

        pub fn setName(&self, value : &str) {
            self._builder.setTextField(0, value);
        }

        pub fn getEmployment(&self) -> Employment::Builder {
            Employment::Builder::new(self._builder)
        }
    }

    pub mod Employment {
        use layout::*;

        pub struct Builder {
            _builder : StructBuilder
        }

        impl Builder {
            pub fn new(builder : StructBuilder) -> Builder {
                Builder { _builder : builder }
            }

            pub fn setUnemployed(&self) {
                self._builder.setDataField::<u16>(2, 0);
            }

            pub fn setEmployer(&self, value : &str) {
                self._builder.setDataField::<u16>(2, 1);
                self._builder.setTextField(3, value);
            }

            pub fn setSchool(&self, value : &str) {
                self._builder.setDataField::<u16>(2, 2);
                self._builder.setTextField(3, value);
            }

            pub fn setSelfEmployed(&self) {
                self._builder.setDataField::<u16>(2, 3);
            }
        }
    }

    pub mod PhoneNumber {
        use layout::*;
//        use addressbook_capnp::*;

        pub struct Reader<'self> {
            _reader : StructReader<'self>
        }

        impl <'self> Reader<'self> {

            pub fn new<'a>(reader : StructReader<'a>) -> Reader<'a> {
                Reader{ _reader : reader }
            }

            pub fn totalSizeInWords(&self) -> uint {
                self._reader.totalSize() as uint
            }
        }

        pub struct Builder {
            _builder : StructBuilder
        }

        impl Builder {
            pub fn new(builder : StructBuilder) -> Builder {
                Builder { _builder : builder }
            }

        }

        pub static STRUCT_SIZE : StructSize =
            StructSize {data : 1, pointers : 1,
                        preferredListEncoding : INLINE_COMPOSITE};

        list_submodule!(addressbook_capnp, Person::PhoneNumber)
    }
}

pub mod AddressBook {
    use layout::*;
    use addressbook_capnp::*;

    pub struct Reader<'self> {
        _reader : StructReader<'self>
    }

    impl <'self> Reader<'self> {

        pub fn new<'a>(reader : StructReader<'a>) -> Reader<'a> {
            Reader{ _reader : reader }
        }

        pub fn totalSizeInWords(&self) -> uint {
            self._reader.totalSize() as uint
        }

        pub fn getPeople(&self) -> Person::List::Reader<'self> {
            Person::List::Reader::new(self._reader.getListField(0, INLINE_COMPOSITE, 0))
        }
    }

    pub struct Builder {
        _builder : StructBuilder
    }

    impl Builder {
        pub fn new(builder : StructBuilder) -> Builder {
            Builder { _builder : builder }
        }

/*
        pub fn initRoot() -> Builder {
        }
*/

        pub fn initPeople(&self, size : uint) -> Person::List::Builder {
            Person::List::Builder::new(
                self._builder.initStructListField(0, size, Person::STRUCT_SIZE))
        }

    }

    pub static STRUCT_SIZE : StructSize = StructSize {data : 0, pointers : 1,
                                                      preferredListEncoding : POINTER};

    list_submodule!(addressbook_capnp, AddressBook)
}