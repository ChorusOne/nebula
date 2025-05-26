// @generated
impl serde::Serialize for AbciResponsesInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.legacy_abci_responses.is_some() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.finalize_block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.ABCIResponsesInfo", len)?;
        if let Some(v) = self.legacy_abci_responses.as_ref() {
            struct_ser.serialize_field("legacyAbciResponses", v)?;
        }
        if self.height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.finalize_block.as_ref() {
            struct_ser.serialize_field("finalizeBlock", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AbciResponsesInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "legacy_abci_responses",
            "legacyAbciResponses",
            "height",
            "finalize_block",
            "finalizeBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LegacyAbciResponses,
            Height,
            FinalizeBlock,
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
                            "legacyAbciResponses" | "legacy_abci_responses" => Ok(GeneratedField::LegacyAbciResponses),
                            "height" => Ok(GeneratedField::Height),
                            "finalizeBlock" | "finalize_block" => Ok(GeneratedField::FinalizeBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AbciResponsesInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.ABCIResponsesInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AbciResponsesInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut legacy_abci_responses__ = None;
                let mut height__ = None;
                let mut finalize_block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LegacyAbciResponses => {
                            if legacy_abci_responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("legacyAbciResponses"));
                            }
                            legacy_abci_responses__ = map_.next_value()?;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FinalizeBlock => {
                            if finalize_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalizeBlock"));
                            }
                            finalize_block__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AbciResponsesInfo {
                    legacy_abci_responses: legacy_abci_responses__,
                    height: height__.unwrap_or_default(),
                    finalize_block: finalize_block__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.ABCIResponsesInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConsensusParamsInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consensus_params.is_some() {
            len += 1;
        }
        if self.last_height_changed != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.ConsensusParamsInfo", len)?;
        if let Some(v) = self.consensus_params.as_ref() {
            struct_ser.serialize_field("consensusParams", v)?;
        }
        if self.last_height_changed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastHeightChanged", ToString::to_string(&self.last_height_changed).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConsensusParamsInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consensus_params",
            "consensusParams",
            "last_height_changed",
            "lastHeightChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConsensusParams,
            LastHeightChanged,
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
                            "consensusParams" | "consensus_params" => Ok(GeneratedField::ConsensusParams),
                            "lastHeightChanged" | "last_height_changed" => Ok(GeneratedField::LastHeightChanged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConsensusParamsInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.ConsensusParamsInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConsensusParamsInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consensus_params__ = None;
                let mut last_height_changed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ConsensusParams => {
                            if consensus_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParams"));
                            }
                            consensus_params__ = map_.next_value()?;
                        }
                        GeneratedField::LastHeightChanged => {
                            if last_height_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastHeightChanged"));
                            }
                            last_height_changed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ConsensusParamsInfo {
                    consensus_params: consensus_params__,
                    last_height_changed: last_height_changed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.ConsensusParamsInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LegacyAbciResponses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deliver_txs.is_empty() {
            len += 1;
        }
        if self.end_block.is_some() {
            len += 1;
        }
        if self.begin_block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.LegacyABCIResponses", len)?;
        if !self.deliver_txs.is_empty() {
            struct_ser.serialize_field("deliverTxs", &self.deliver_txs)?;
        }
        if let Some(v) = self.end_block.as_ref() {
            struct_ser.serialize_field("endBlock", v)?;
        }
        if let Some(v) = self.begin_block.as_ref() {
            struct_ser.serialize_field("beginBlock", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LegacyAbciResponses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deliver_txs",
            "deliverTxs",
            "end_block",
            "endBlock",
            "begin_block",
            "beginBlock",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DeliverTxs,
            EndBlock,
            BeginBlock,
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
                            "deliverTxs" | "deliver_txs" => Ok(GeneratedField::DeliverTxs),
                            "endBlock" | "end_block" => Ok(GeneratedField::EndBlock),
                            "beginBlock" | "begin_block" => Ok(GeneratedField::BeginBlock),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LegacyAbciResponses;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.LegacyABCIResponses")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LegacyAbciResponses, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut deliver_txs__ = None;
                let mut end_block__ = None;
                let mut begin_block__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DeliverTxs => {
                            if deliver_txs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deliverTxs"));
                            }
                            deliver_txs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndBlock => {
                            if end_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endBlock"));
                            }
                            end_block__ = map_.next_value()?;
                        }
                        GeneratedField::BeginBlock => {
                            if begin_block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("beginBlock"));
                            }
                            begin_block__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LegacyAbciResponses {
                    deliver_txs: deliver_txs__.unwrap_or_default(),
                    end_block: end_block__,
                    begin_block: begin_block__,
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.LegacyABCIResponses", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseBeginBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.ResponseBeginBlock", len)?;
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseBeginBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Events,
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
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseBeginBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.ResponseBeginBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseBeginBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseBeginBlock {
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.ResponseBeginBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResponseEndBlock {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_updates.is_empty() {
            len += 1;
        }
        if self.consensus_param_updates.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.ResponseEndBlock", len)?;
        if !self.validator_updates.is_empty() {
            struct_ser.serialize_field("validatorUpdates", &self.validator_updates)?;
        }
        if let Some(v) = self.consensus_param_updates.as_ref() {
            struct_ser.serialize_field("consensusParamUpdates", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResponseEndBlock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_updates",
            "validatorUpdates",
            "consensus_param_updates",
            "consensusParamUpdates",
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorUpdates,
            ConsensusParamUpdates,
            Events,
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
                            "validatorUpdates" | "validator_updates" => Ok(GeneratedField::ValidatorUpdates),
                            "consensusParamUpdates" | "consensus_param_updates" => Ok(GeneratedField::ConsensusParamUpdates),
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResponseEndBlock;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.ResponseEndBlock")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResponseEndBlock, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator_updates__ = None;
                let mut consensus_param_updates__ = None;
                let mut events__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
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
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResponseEndBlock {
                    validator_updates: validator_updates__.unwrap_or_default(),
                    consensus_param_updates: consensus_param_updates__,
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.ResponseEndBlock", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.version.is_some() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.initial_height != 0 {
            len += 1;
        }
        if self.last_block_height != 0 {
            len += 1;
        }
        if self.last_block_id.is_some() {
            len += 1;
        }
        if self.last_block_time.is_some() {
            len += 1;
        }
        if self.next_validators.is_some() {
            len += 1;
        }
        if self.validators.is_some() {
            len += 1;
        }
        if self.last_validators.is_some() {
            len += 1;
        }
        if self.last_height_validators_changed != 0 {
            len += 1;
        }
        if self.consensus_params.is_some() {
            len += 1;
        }
        if self.last_height_consensus_params_changed != 0 {
            len += 1;
        }
        if !self.last_results_hash.is_empty() {
            len += 1;
        }
        if !self.app_hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.State", len)?;
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.initial_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("initialHeight", ToString::to_string(&self.initial_height).as_str())?;
        }
        if self.last_block_height != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastBlockHeight", ToString::to_string(&self.last_block_height).as_str())?;
        }
        if let Some(v) = self.last_block_id.as_ref() {
            struct_ser.serialize_field("lastBlockId", v)?;
        }
        if let Some(v) = self.last_block_time.as_ref() {
            struct_ser.serialize_field("lastBlockTime", v)?;
        }
        if let Some(v) = self.next_validators.as_ref() {
            struct_ser.serialize_field("nextValidators", v)?;
        }
        if let Some(v) = self.validators.as_ref() {
            struct_ser.serialize_field("validators", v)?;
        }
        if let Some(v) = self.last_validators.as_ref() {
            struct_ser.serialize_field("lastValidators", v)?;
        }
        if self.last_height_validators_changed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastHeightValidatorsChanged", ToString::to_string(&self.last_height_validators_changed).as_str())?;
        }
        if let Some(v) = self.consensus_params.as_ref() {
            struct_ser.serialize_field("consensusParams", v)?;
        }
        if self.last_height_consensus_params_changed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastHeightConsensusParamsChanged", ToString::to_string(&self.last_height_consensus_params_changed).as_str())?;
        }
        if !self.last_results_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastResultsHash", pbjson::private::base64::encode(&self.last_results_hash).as_str())?;
        }
        if !self.app_hash.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("appHash", pbjson::private::base64::encode(&self.app_hash).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "chain_id",
            "chainId",
            "initial_height",
            "initialHeight",
            "last_block_height",
            "lastBlockHeight",
            "last_block_id",
            "lastBlockId",
            "last_block_time",
            "lastBlockTime",
            "next_validators",
            "nextValidators",
            "validators",
            "last_validators",
            "lastValidators",
            "last_height_validators_changed",
            "lastHeightValidatorsChanged",
            "consensus_params",
            "consensusParams",
            "last_height_consensus_params_changed",
            "lastHeightConsensusParamsChanged",
            "last_results_hash",
            "lastResultsHash",
            "app_hash",
            "appHash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            ChainId,
            InitialHeight,
            LastBlockHeight,
            LastBlockId,
            LastBlockTime,
            NextValidators,
            Validators,
            LastValidators,
            LastHeightValidatorsChanged,
            ConsensusParams,
            LastHeightConsensusParamsChanged,
            LastResultsHash,
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
                            "version" => Ok(GeneratedField::Version),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "initialHeight" | "initial_height" => Ok(GeneratedField::InitialHeight),
                            "lastBlockHeight" | "last_block_height" => Ok(GeneratedField::LastBlockHeight),
                            "lastBlockId" | "last_block_id" => Ok(GeneratedField::LastBlockId),
                            "lastBlockTime" | "last_block_time" => Ok(GeneratedField::LastBlockTime),
                            "nextValidators" | "next_validators" => Ok(GeneratedField::NextValidators),
                            "validators" => Ok(GeneratedField::Validators),
                            "lastValidators" | "last_validators" => Ok(GeneratedField::LastValidators),
                            "lastHeightValidatorsChanged" | "last_height_validators_changed" => Ok(GeneratedField::LastHeightValidatorsChanged),
                            "consensusParams" | "consensus_params" => Ok(GeneratedField::ConsensusParams),
                            "lastHeightConsensusParamsChanged" | "last_height_consensus_params_changed" => Ok(GeneratedField::LastHeightConsensusParamsChanged),
                            "lastResultsHash" | "last_results_hash" => Ok(GeneratedField::LastResultsHash),
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
            type Value = State;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.State")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<State, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut chain_id__ = None;
                let mut initial_height__ = None;
                let mut last_block_height__ = None;
                let mut last_block_id__ = None;
                let mut last_block_time__ = None;
                let mut next_validators__ = None;
                let mut validators__ = None;
                let mut last_validators__ = None;
                let mut last_height_validators_changed__ = None;
                let mut consensus_params__ = None;
                let mut last_height_consensus_params_changed__ = None;
                let mut last_results_hash__ = None;
                let mut app_hash__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map_.next_value()?;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InitialHeight => {
                            if initial_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialHeight"));
                            }
                            initial_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastBlockHeight => {
                            if last_block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastBlockHeight"));
                            }
                            last_block_height__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastBlockId => {
                            if last_block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastBlockId"));
                            }
                            last_block_id__ = map_.next_value()?;
                        }
                        GeneratedField::LastBlockTime => {
                            if last_block_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastBlockTime"));
                            }
                            last_block_time__ = map_.next_value()?;
                        }
                        GeneratedField::NextValidators => {
                            if next_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextValidators"));
                            }
                            next_validators__ = map_.next_value()?;
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = map_.next_value()?;
                        }
                        GeneratedField::LastValidators => {
                            if last_validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastValidators"));
                            }
                            last_validators__ = map_.next_value()?;
                        }
                        GeneratedField::LastHeightValidatorsChanged => {
                            if last_height_validators_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastHeightValidatorsChanged"));
                            }
                            last_height_validators_changed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ConsensusParams => {
                            if consensus_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensusParams"));
                            }
                            consensus_params__ = map_.next_value()?;
                        }
                        GeneratedField::LastHeightConsensusParamsChanged => {
                            if last_height_consensus_params_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastHeightConsensusParamsChanged"));
                            }
                            last_height_consensus_params_changed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastResultsHash => {
                            if last_results_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastResultsHash"));
                            }
                            last_results_hash__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
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
                Ok(State {
                    version: version__,
                    chain_id: chain_id__.unwrap_or_default(),
                    initial_height: initial_height__.unwrap_or_default(),
                    last_block_height: last_block_height__.unwrap_or_default(),
                    last_block_id: last_block_id__,
                    last_block_time: last_block_time__,
                    next_validators: next_validators__,
                    validators: validators__,
                    last_validators: last_validators__,
                    last_height_validators_changed: last_height_validators_changed__.unwrap_or_default(),
                    consensus_params: consensus_params__,
                    last_height_consensus_params_changed: last_height_consensus_params_changed__.unwrap_or_default(),
                    last_results_hash: last_results_hash__.unwrap_or_default(),
                    app_hash: app_hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.State", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ValidatorsInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.validator_set.is_some() {
            len += 1;
        }
        if self.last_height_changed != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.ValidatorsInfo", len)?;
        if let Some(v) = self.validator_set.as_ref() {
            struct_ser.serialize_field("validatorSet", v)?;
        }
        if self.last_height_changed != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lastHeightChanged", ToString::to_string(&self.last_height_changed).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorsInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_set",
            "validatorSet",
            "last_height_changed",
            "lastHeightChanged",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorSet,
            LastHeightChanged,
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
                            "validatorSet" | "validator_set" => Ok(GeneratedField::ValidatorSet),
                            "lastHeightChanged" | "last_height_changed" => Ok(GeneratedField::LastHeightChanged),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorsInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.ValidatorsInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ValidatorsInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut validator_set__ = None;
                let mut last_height_changed__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ValidatorSet => {
                            if validator_set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validatorSet"));
                            }
                            validator_set__ = map_.next_value()?;
                        }
                        GeneratedField::LastHeightChanged => {
                            if last_height_changed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastHeightChanged"));
                            }
                            last_height_changed__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ValidatorsInfo {
                    validator_set: validator_set__,
                    last_height_changed: last_height_changed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.ValidatorsInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Version {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.consensus.is_some() {
            len += 1;
        }
        if !self.software.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("cometbft.state.v1.Version", len)?;
        if let Some(v) = self.consensus.as_ref() {
            struct_ser.serialize_field("consensus", v)?;
        }
        if !self.software.is_empty() {
            struct_ser.serialize_field("software", &self.software)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Version {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "consensus",
            "software",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Consensus,
            Software,
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
                            "consensus" => Ok(GeneratedField::Consensus),
                            "software" => Ok(GeneratedField::Software),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct cometbft.state.v1.Version")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Version, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut consensus__ = None;
                let mut software__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Consensus => {
                            if consensus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("consensus"));
                            }
                            consensus__ = map_.next_value()?;
                        }
                        GeneratedField::Software => {
                            if software__.is_some() {
                                return Err(serde::de::Error::duplicate_field("software"));
                            }
                            software__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Version {
                    consensus: consensus__,
                    software: software__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("cometbft.state.v1.Version", FIELDS, GeneratedVisitor)
    }
}
