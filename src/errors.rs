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
        WrongSimpleListTarsTypeErr {
            description("decoder: wrong simple list type")
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

