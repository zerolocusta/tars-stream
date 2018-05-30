quick_error!{
    #[derive(Debug, PartialEq, Eq)]
    pub enum DecodeErr{
        NoEnoughDataErr{
            description("decoder: without enough data to read")
        }
        UnknownTarsTypeErr{
            description("decoder: unknown tars type")
        }
    }
}

