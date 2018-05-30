quick_error!{
    #[derive(Debug)]
    pub enum DecodeErr{
        NoEnoughDataErr{
            description("decoder: without enough data to read")
        }
    }
}

