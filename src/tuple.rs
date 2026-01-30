use super::error::Error;
use super::ser::Serializer;

use serde::{Serialize, ser};

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += " ]";
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += " ]";
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with('[') {
            self.output += " ";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.output += " ] }";
        Ok(())
    }
}
