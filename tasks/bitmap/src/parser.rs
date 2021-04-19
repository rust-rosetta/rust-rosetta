use super::{Color, ImageFormat};
use std::str::from_utf8;
use std::str::FromStr;

pub fn parse_version(input: &[u8]) -> nom::IResult<&[u8], ImageFormat> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::line_ending;
    use nom::combinator::map;
    use nom::sequence::terminated;

    // starts with P3/P6 ends with a CR/LF
    terminated(
        alt((
            map(tag("P3".as_bytes()), |_| ImageFormat::P3),
            map(tag("P6".as_bytes()), |_| ImageFormat::P6),
        )),
        line_ending,
    )(input)
}

pub fn parse_image_attributes(input: &[u8]) -> nom::IResult<&[u8], (usize, usize, usize)> {
    use nom::character::complete::line_ending;
    use nom::character::complete::{digit1, space1};
    use nom::sequence::terminated;
    use nom::sequence::tuple;

    // 3 numbers separated by spaces ends with a CR/LF
    terminated(tuple((digit1, space1, digit1, space1, digit1)), line_ending)(input).map(
        |(next_input, result)| {
            (
                next_input,
                (
                    usize::from_str_radix(from_utf8(result.0).unwrap(), 10).unwrap(),
                    usize::from_str_radix(from_utf8(result.2).unwrap(), 10).unwrap(),
                    usize::from_str_radix(from_utf8(result.4).unwrap(), 10).unwrap(),
                ),
            )
        },
    )
}

pub fn parse_color_binary(input: &[u8]) -> nom::IResult<&[u8], Color> {
    use nom::number::complete::u8 as nom_u8;
    use nom::sequence::tuple;

    tuple((nom_u8, nom_u8, nom_u8))(input).map(|(next_input, res)| {
        (
            next_input,
            Color {
                red: res.0,
                green: res.1,
                blue: res.2,
            },
        )
    })
}

pub fn parse_data_binary(input: &[u8]) -> nom::IResult<&[u8], Vec<Color>> {
    use nom::multi::many0;
    many0(parse_color_binary)(input)
}

pub fn parse_color_ascii(input: &[u8]) -> nom::IResult<&[u8], Color> {
    use nom::character::complete::{digit1, space0, space1};
    use nom::sequence::tuple;

    tuple((digit1, space1, digit1, space1, digit1, space0))(input).map(|(next_input, res)| {
        (
            next_input,
            Color {
                red: u8::from_str(from_utf8(res.0).unwrap()).unwrap(),
                green: u8::from_str(from_utf8(res.2).unwrap()).unwrap(),
                blue: u8::from_str(from_utf8(res.4).unwrap()).unwrap(),
            },
        )
    })
}

pub fn parse_data_ascii(input: &[u8]) -> nom::IResult<&[u8], Vec<Color>> {
    use nom::multi::many0;
    many0(parse_color_ascii)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;

    #[test]
    fn test_parse_version() {
        assert_eq!(
            Ok((&b"200 300 255\nabcde"[..], ImageFormat::P6)),
            parse_version(&b"P6\n200 300 255\nabcde"[..])
        );

        assert_eq!(
            Ok((&b"200 300 255\nabcde"[..], ImageFormat::P3)),
            parse_version(&b"P3\n200 300 255\nabcde"[..])
        );

        assert_eq!(
            Err(Err::Error(nom::error::Error::new(
                &b"P1\n200 300 255\nabcde"[..],
                nom::error::ErrorKind::Tag,
            ))),
            parse_version(&b"P1\n200 300 255\nabcde"[..])
        );

        assert_eq!(
            Err(Err::Error(nom::error::Error::new(
                &b"\n200 300 255\nabcde"[..],
                nom::error::ErrorKind::Tag,
            ))),
            parse_version(&b"\n200 300 255\nabcde"[..])
        );
    }

    #[test]
    fn test_parse_image_attributes() {
        assert_eq!(
            Ok((&b"abcde"[..], (200, 300, 255))),
            parse_image_attributes(&b"200 300 255\nabcde"[..])
        );

        assert_eq!(
            Err(Err::Error(nom::error::Error::new(
                &b"a200 300 255\nabcde"[..],
                nom::error::ErrorKind::Digit,
            ))),
            parse_image_attributes(&b"a200 300 255\nabcde"[..])
        );

        assert_eq!(
            Ok((&b"abcde"[..], (200, 300, 255))),
            parse_image_attributes(&b"200    300     255\nabcde"[..])
        );
    }

    #[test]
    fn test_parse_color_binary() {
        assert_eq!(
            Ok((
                &b""[..],
                Color {
                    red: 255,
                    green: 0,
                    blue: 0
                }
            )),
            parse_color_binary(&[255, 0, 0])
        );
    }

    #[test]
    fn test_parse_data_binary() {
        assert_eq!(
            Ok((
                &b""[..],
                vec![
                    Color {
                        red: 255,
                        green: 0,
                        blue: 0
                    },
                    Color {
                        red: 0,
                        green: 0,
                        blue: 255
                    }
                ]
            )),
            parse_data_binary(&[255, 0, 0, 0, 0, 255])
        );
    }

    #[test]
    fn test_parse_color_ascii() {
        assert_eq!(
            Ok((
                &b""[..],
                Color {
                    red: 255,
                    green: 0,
                    blue: 0
                }
            )),
            parse_color_ascii(&b"255 0 0"[..])
        );
    }

    #[test]
    fn test_parse_data_ascii() {
        assert_eq!(
            Ok((
                &b""[..],
                vec![
                    Color {
                        red: 255,
                        green: 0,
                        blue: 0
                    },
                    Color {
                        red: 0,
                        green: 0,
                        blue: 255
                    }
                ]
            )),
            parse_data_ascii(&b"255 0 0 0 0 255"[..])
        );
    }
}
