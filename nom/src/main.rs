#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn hex_primary(input: &str) -> nom::IResult<&str, u8> {
    nom::combinator::map_res(
        nom::bytes::complete::take_while_m_n(2, 2, |c: char| c.is_digit(16)),
        |x: &str| u8::from_str_radix(x, 16),
    )(input)
}

fn parse_color(input: &str) -> nom::IResult<&str, Color> {
    let (input, _) = nom::bytes::complete::tag("#")(input)?;
    let (input, (red, green, blue)) =
        nom::sequence::tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

#[test]
fn test() {
    assert_eq!(
        parse_color("#012345"),
        Ok((
            "",
            Color {
                red: 0x01,
                green: 0x23,
                blue: 0x45,
            },
        )),
    );

    assert_eq!(
        parse_color("#00ff80"),
        Ok((
            "",
            Color {
                red: 0x00,
                green: 0xff,
                blue: 0x80,
            },
        )),
    );
}

fn main() {
    println!("{:?}", parse_color("#1289ef"));
}
