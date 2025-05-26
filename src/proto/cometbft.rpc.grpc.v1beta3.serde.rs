// @generated
impl serde::Serialize for ResponseBroadcastTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check_tx.is_some() {
            len += 1;
        }
        if self.tx_result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.rpc.grpc.v1beta3.ResponseBroadcastTx", len)?;
        if let Some(v) = self.check_tx.as_ref() {
            struct_ser.serialize_field("checkTx", v)?;
        }
        if let Some(v) = self.tx_result.as_ref() {
            struct_ser.serialize_field("txResult", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseBroadcastTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "check_tx",
            "checkTx",
            "tx_result",
            "txResult",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CheckTx,
            TxResult,
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
                            "checkTx" | "check_tx" => Ok(GeneratedField::CheckTx),
                            "txResult" | "tx_result" => Ok(GeneratedField::TxResult),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseBroadcastTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.rpc.grpc.v1beta3.ResponseBroadcastTx")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseBroadcastTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check_tx__ = None;
                let mut tx_result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CheckTx => {
                            if check_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("checkTx"));
                            }
                            check_tx__ = map_.next_value()?;
                        }
                        GeneratedField::TxResult => {
                            if tx_result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txResult"));
                            }
                            tx_result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ResponseBroadcastTx {
                    check_tx: check_tx__,
                    tx_result: tx_result__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.rpc.grpc.v1beta3.ResponseBroadcastTx", FIELDS, GeneratedVisitor)
    }
}
