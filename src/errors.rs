quick_error!{
    #[derive(Debug, PartialEq, Eq)]
    pub enum DecodeErr{
        NoEnoughDataErr{
            description("decoder: without enough data to read")
        }
        UnknownTarsTypeErr{
            description("decoder: unknown tars type")
        }
        TagNotFoundErr{
            description("decoder: Tag Not Found")
        }
        MisMatchTarsTypeErr {
            description("decoder: mismatch type")
        }
        WrongSimpleListTarsTypeErr {
            description("decoder: wrong simple list type")
        }
        InvalidEnumValue {
            description("decoder: invalid enum value")
        }
        FieldNotFoundErr(desc: String) {
            description(desc)
        }
        TypeNotFoundErr(desc: String) {
            description(desc)
        }
    }
}

quick_error!{
    #[derive(Debug, PartialEq, Eq)]
    pub enum TarsTypeErr{
        DisMatchTarsTypeErr{
            description("tars_type: disMatch tars_type")
        }
    }
}

quick_error!{
    #[derive(Debug, PartialEq, Eq)]
    pub enum EncodeErr{
        TooBigTagErr{
            description("encoder: tag too big, max value is 255")
        }
        ConvertU8Err{
            description("encoder: cannot convert to u8")
        }
        BufferTooBigErr {
            description("encoder: BufferTooBigErr len bigger than 4294967295 bytes")
        }
        UnknownTarsTypeErr{
            description("encoder: unknown tars type")
        }
    }
}
