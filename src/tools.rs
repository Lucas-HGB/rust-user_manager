pub(crate) fn parse_input_to_u8(input: String) -> u8 {
    input.trim().parse::<u8>().expect("Please type a number!") as u8
}

pub(crate) fn parse_input_to_u32(input: String) -> u32 {
    input.trim().parse::<u32>().expect("Please type a number!") as u32
}