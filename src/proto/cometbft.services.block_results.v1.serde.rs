// @generated
impl serde::Serialize for GetBlockResultsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.services.block_results.v1.GetBlockResultsRequest", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBlockResultsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
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
                            "height" => Ok(GeneratedField::Height),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBlockResultsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.services.block_results.v1.GetBlockResultsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBlockResultsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetBlockResultsRequest {
                    height: height__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.services.block_results.v1.GetBlockResultsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBlockResultsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.height != 0 {
            len += 1;
        }
        if !self.tx_results.is_empty() {
            len += 1;
        }
        if !self.finalize_block_events.is_empty() {
            len += 1;
        }
        if !self.validator_updates.is_empty() {
            len += 1;
        }
        if self.consensus_param_updates.is_some() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.services.block_results.v1.GetBlockResultsResponse", len)?;
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if !self.tx_results.is_empty() {
            struct_ser.serialize_field("txResults", &self.tx_results)?;
        }
        if !self.finalize_block_events.is_empty() {
            struct_ser.serialize_field("finalizeBlockEvents", &self.finalize_block_events)?;
        }
        if !self.validator_updates.is_empty() {
            struct_ser.serialize_field("validatorUpdates", &self.validator_updates)?;
        }
        if let Some(v) = self.consensus_param_updates.as_ref() {
            struct_ser.serialize_field("consensusParamUpdates", v)?;
        }
        if !self.app_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("appHash", pbjson::private::base64::encode(&self.app_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBlockResultsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "height",
            "tx_results",
            "txResults",
            "finalize_block_events",
            "finalizeBlockEvents",
            "validator_updates",
            "validatorUpdates",
            "consensus_param_updates",
            "consensusParamUpdates",
            "app_hash",
            "appHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Height,
            TxResults,
            FinalizeBlockEvents,
            ValidatorUpdates,
            ConsensusParamUpdates,
            AppHash,
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
                            "height" => Ok(GeneratedField::Height),
                            "txResults" | "tx_results" => Ok(GeneratedField::TxResults),
                            "finalizeBlockEvents" | "finalize_block_events" => Ok(GeneratedField::FinalizeBlockEvents),
                            "validatorUpdates" | "validator_updates" => Ok(GeneratedField::ValidatorUpdates),
                            "consensusParamUpdates" | "consensus_param_updates" => Ok(GeneratedField::ConsensusParamUpdates),
                            "appHash" | "app_hash" => Ok(GeneratedField::AppHash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBlockResultsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.services.block_results.v1.GetBlockResultsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBlockResultsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut height__ = None;
                let mut tx_results__ = None;
                let mut finalize_block_events__ = None;
                let mut validator_updates__ = None;
                let mut consensus_param_updates__ = None;
                let mut app_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TxResults => {
                            if tx_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("txResults"));
                            }
                            tx_results__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FinalizeBlockEvents => {
                            if finalize_block_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalizeBlockEvents"));
                            }
                            finalize_block_events__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ValidatorUpdates => {
                            if validator_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorUpdates"));
                            }
                            validator_updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ConsensusParamUpdates => {
                            if consensus_param_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParamUpdates"));
                            }
                            consensus_param_updates__ = map_.next_value()?;
                        }
                        GeneratedField::AppHash => {
                            if app_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("appHash"));
                            }
                            app_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetBlockResultsResponse {
                    height: height__.unwrap_or_default(),
                    tx_results: tx_results__.unwrap_or_default(),
                    finalize_block_events: finalize_block_events__.unwrap_or_default(),
                    validator_updates: validator_updates__.unwrap_or_default(),
                    consensus_param_updates: consensus_param_updates__,
                    app_hash: app_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.services.block_results.v1.GetBlockResultsResponse", FIELDS, GeneratedVisitor)
    }
}
