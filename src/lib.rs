#![allow(non_camel_case_types)]

pub mod fpoint;

#[cfg(test)]
mod test {
    use crate::fpoint::fi;

    #[test]
    fn formatting() {
        let nb = fi::from(8762092934747);

        assert_eq!("87620.92934747", nb.fmt());
    }

    #[test]
    fn ops() {
        let nb = fi::fixed_from(4, 1);
        let nb2 = fi::fixed_from(2, 1);

        assert_eq!(6, (nb + nb2).value);
        assert_eq!(2, (nb - nb2).value);
        assert_eq!(2, (nb / nb2).value);
        assert_eq!(8, (nb * nb2).value);
    }

    #[test]
    fn froms() {
        let nb = fi::from(4);
        let nb2 = fi::fixed_from(4, 1);

        assert_eq!(nb.value, 4);
        assert_eq!(nb2.value, 4);
    }
}
