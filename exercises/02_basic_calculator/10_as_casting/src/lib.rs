// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct value after the conversion.

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        let v: u32 = 47;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    #[allow(overflowing_literals)]
    fn u8_to_i8() {
        //redo
        //let v: i8 = 255;
        let x: u8 = 255 as i8;
        let y: i8 = -1;
        assert_eq!(x,y);
    }

    #[test]
    fn bool_to_u8() {
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
