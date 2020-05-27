use crate::message;
use crate::params;

pub trait Marshal {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error>;

    fn signature(&self) -> crate::signature::Type;
    fn alignment(&self) -> usize;
}

impl <P: Marshal> Marshal for &P {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        (*self).marshal(byteorder, buf)
    }
    
    fn signature(&self) -> crate::signature::Type {
        (*self).signature()
    }
    fn alignment(&self) -> usize {
        (*self).alignment()
    }
} 

impl Marshal for () {
    fn marshal(
        &self,
        _byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E: Marshal> Marshal for (E,) {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        self.0.marshal(byteorder, buf)?;
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![self
            .0
            .signature()]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E1: Marshal, E2: Marshal> Marshal for (E1, E2) {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        self.0.marshal(byteorder, buf)?;
        self.1.marshal(byteorder, buf)?;
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![
            self.0.signature(),
            self.1.signature(),
        ]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E1: Marshal, E2: Marshal, E3: Marshal> Marshal for (E1, E2, E3) {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        self.0.marshal(byteorder, buf)?;
        self.1.marshal(byteorder, buf)?;
        self.2.marshal(byteorder, buf)?;
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![
            self.0.signature(),
            self.1.signature(),
            self.2.signature(),
        ]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E1: Marshal, E2: Marshal, E3: Marshal, E4: Marshal> Marshal for (E1, E2, E3, E4) {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        self.0.marshal(byteorder, buf)?;
        self.1.marshal(byteorder, buf)?;
        self.2.marshal(byteorder, buf)?;
        self.3.marshal(byteorder, buf)?;
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![
            self.0.signature(),
            self.1.signature(),
            self.2.signature(),
            self.3.signature(),
        ]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E1: Marshal, E2: Marshal, E3: Marshal, E4: Marshal, E5: Marshal> Marshal
    for (E1, E2, E3, E4, E5)
{
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        // always align to 8
        crate::wire::util::pad_to_align(8, buf);
        self.0.marshal(byteorder, buf)?;
        self.1.marshal(byteorder, buf)?;
        self.2.marshal(byteorder, buf)?;
        self.3.marshal(byteorder, buf)?;
        self.4.marshal(byteorder, buf)?;
        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Struct(vec![
            self.0.signature(),
            self.1.signature(),
            self.2.signature(),
            self.3.signature(),
            self.4.signature(),
        ]))
    }

    fn alignment(&self) -> usize {
        8
    }
}

impl<E: Marshal> Marshal for &[E] {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        if self.is_empty() {
            return Err(message::Error::EmptyArray);
        }

        // always align to 4
        crate::wire::util::pad_to_align(4, buf);

        let size_pos = buf.len();
        buf.push(0);
        buf.push(0);
        buf.push(0);
        buf.push(0);

        if self.len() > 0 && self[0].alignment() > 4 {
            let pad_size = buf.len() % self[0].alignment();
            eprintln!("pad_size: {}", pad_size);
            for _ in 0..pad_size {
                buf.push(0);
            }
        }

        let size_before = buf.len();
        for p in self.iter() {
            p.marshal(byteorder, buf)?;
        }
        let size_of_content = buf.len() - size_before;
        crate::wire::util::insert_u32(
            byteorder,
            size_of_content as u32,
            &mut buf[size_pos..size_pos + 4],
        );

        Ok(())
    }
    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Container(crate::signature::Container::Array(Box::new(
            self[0].signature(),
        )))
    }

    fn alignment(&self) -> usize {
        4
    }
}

impl<K: Marshal, V: Marshal> Marshal for &std::collections::HashMap<K, V> {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        if self.is_empty() {
            return Err(message::Error::EmptyDict);
        }

        // always align to 4
        crate::wire::util::pad_to_align(4, buf);

        let size_pos = buf.len();
        buf.push(0);
        buf.push(0);
        buf.push(0);
        buf.push(0);

        if self.len() > 0 {
            // always align to 8
            crate::wire::util::pad_to_align(8, buf);
        }

        let size_before = buf.len();
        for p in self.iter() {
            // always align to 8
            crate::wire::util::pad_to_align(8, buf);
            p.0.marshal(byteorder, buf)?;
            p.1.marshal(byteorder, buf)?;
        }
        let size_of_content = buf.len() - size_before;
        crate::wire::util::insert_u32(
            byteorder,
            size_of_content as u32,
            &mut buf[size_pos..size_pos + 4],
        );

        Ok(())
    }

    fn signature(&self) -> crate::signature::Type {
        let ks = self.keys().nth(0).unwrap().signature();
        let vs = self.values().nth(0).unwrap().signature();
        if let crate::signature::Type::Base(ks) = ks {
            crate::signature::Type::Container(crate::signature::Container::Dict(ks, Box::new(vs)))
        } else {
            panic!("Ivalid key sig")
        }
    }

    fn alignment(&self) -> usize {
        4
    }
}

impl<'a> Marshal for params::Param<'a, 'a> {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        crate::wire::marshal_container::marshal_param(self, byteorder, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        self.sig()
    }
    fn alignment(&self) -> usize {
        self.sig().get_alignment()
    }
}

impl<'a> Marshal for params::Base<'a> {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        crate::wire::marshal_base::marshal_base_param(byteorder, self, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        self.sig()
    }
    fn alignment(&self) -> usize {
        self.sig().get_alignment()
    }
}

impl Marshal for u64 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Uint64)
    }
    fn alignment(&self) -> usize {
        let b: params::Base = self.into();
        b.sig().get_alignment()
    }
}
impl Marshal for i64 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Int64)
    }
    fn alignment(&self) -> usize {
        let b: params::Base = self.into();
        b.sig().get_alignment()
    }
}

impl Marshal for u32 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Uint32)
    }
    fn alignment(&self) -> usize {
        let b: params::Base = self.into();
        b.sig().get_alignment()
    }
}
impl Marshal for i32 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Int32)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}

impl Marshal for u16 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Uint16)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}
impl Marshal for i16 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Int16)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}

impl Marshal for u8 {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Byte)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}

impl Marshal for bool {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        let b: params::Base = self.into();
        crate::wire::marshal_base::marshal_base_param(byteorder, &b, buf)
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::Boolean)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}

impl Marshal for String {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        crate::wire::util::write_string(self.as_str(), byteorder, buf);
        Ok(())
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::String)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}

impl Marshal for &str {
    fn marshal(
        &self,
        byteorder: message::ByteOrder,
        buf: &mut Vec<u8>,
    ) -> Result<(), message::Error> {
        crate::wire::util::write_string(self, byteorder, buf);
        Ok(())
    }

    fn signature(&self) -> crate::signature::Type {
        crate::signature::Type::Base(crate::signature::Base::String)
    }
    fn alignment(&self) -> usize {
        self.signature().get_alignment()
    }
}
