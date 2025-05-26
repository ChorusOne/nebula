// @generated
impl serde::Serialize for GetVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("cometbft.services.version.v1.GetVersionRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.services.version.v1.GetVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(GetVersionRequest {
                })
            }
        }
        deserializer.deserialize_struct("cometbft.services.version.v1.GetVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.node.is_empty() {
            len += 1;
        }
        if !self.abci.is_empty() {
            len += 1;
        }
        if self.p2p != 0 {
            len += 1;
        }
        if self.block != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.services.version.v1.GetVersionResponse", len)?;
        if !self.node.is_empty() {
            struct_ser.serialize_field("node", &self.node)?;
        }
        if !self.abci.is_empty() {
            struct_ser.serialize_field("abci", &self.abci)?;
        }
        if self.p2p != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("p2p", ToString::to_string(&self.p2p).as_str())?;
        }
        if self.block != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("block", ToString::to_string(&self.block).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "node",
            "abci",
            "p2p",
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Node,
            Abci,
            P2p,
            Block,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "node" => Ok(GeneratedField::Node),
                            "abci" => Ok(GeneratedField::Abci),
                            "p2p" => Ok(GeneratedField::P2p),
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.services.version.v1.GetVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut node__ = None;
                let mut abci__ = None;
                let mut p2p__ = None;
                let mut block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Node => {
                            if node__.is_some() {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            node__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Abci => {
                            if abci__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abci"));
                            }
                            abci__ = Some(map_.next_value()?);
                        }
                        GeneratedField::P2p => {
                            if p2p__.is_some() {
                                return Err(serde::de::Error::duplicate_field("p2p"));
                            }
                            p2p__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetVersionResponse {
                    node: node__.unwrap_or_default(),
                    abci: abci__.unwrap_or_default(),
                    p2p: p2p__.unwrap_or_default(),
                    block: block__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.services.version.v1.GetVersionResponse", FIELDS, GeneratedVisitor)
    }
}
